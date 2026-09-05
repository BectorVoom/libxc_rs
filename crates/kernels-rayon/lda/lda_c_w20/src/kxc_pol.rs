//! LDA_C_W20 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_w20.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_w20_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = (simd::ln(f64x8::splat(2.0)));
            let t2 = f64x8::splat(1.0) - t1;
            let t3 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = t1 / f64x8::splat(6.0);
            let t8 = f64x8::splat(1.0) / t2;
            let t12 = (simd::exp(-f64x8::splat(2.0) * (-f64x8::splat(0.16244537117517982) + t6) * t8 * t3));
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t16 = (simd::cbrt(t15));
            let t17 = t16 * t16;
            let t18 = t14 * t17;
            let t19 = f64x8::splat(M_CBRT4);
            let t20 = v_rho0 + v_rho1;
            let t21 = (simd::cbrt(t20));
            let t22 = t21 * t21;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t18 * t19 * t23;
            let t27 = (simd::exp(-t25 / f64x8::splat(40000.0)));
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = f64x8::splat(M_CBRTPI);
            let t30 = t29 * t29;
            let t32 = (simd::cbrt(f64x8::splat(9.0)));
            let t33 = f64x8::splat(1.0) / t30 * t32;
            let t34 = t19 * t19;
            let t40 = t12 / f64x8::splat(2.0);
            let t41 = (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t33 * t34) * t8 * t3 + t40;
            let t45 = (-f64x8::splat(2.0) * t28 * t41 + t12) * t14;
            let t46 = f64x8::splat(1.0) / t16;
            let t47 = t46 * t19;
            let t48 = t47 * t21;
            let t51 = t28 * t8;
            let t52 = ((f64x8::splat(4.0)).sqrt());
            let t53 = t13 * t16;
            let t54 = f64x8::splat(1.0) / t21;
            let t56 = t53 * t34 * t54;
            let t57 = ((t56).sqrt());
            let t59 = f64x8::splat(1.0) / t57 / t56;
            let t61 = t51 * t52 * t59;
            let t63 = t32 * t32;
            let t64 = t63 * t19;
            let t65 = t30 * t3;
            let t69 = -f64x8::splat(3.0) / f64x8::splat(40.0) * t64 * t65 * t8 + t40;
            let t73 = (-f64x8::splat(2.0) * t28 * t69 + t12) * t13;
            let t74 = f64x8::splat(1.0) / t17;
            let t75 = t74 * t34;
            let t76 = t75 * t22;
            let t79 = f64x8::splat(1.0) + t45 * t48 / f64x8::splat(3.0) - f64x8::splat(118.43525281307231) * t61 + t73 * t76 / f64x8::splat(3.0);
            let t80 = (simd::ln(t79));
            let t82 = t5 * t80 / f64x8::splat(2.0);
            let t83 = t53 * t34;
            let t84 = t54 * t27;
            let t85 = ((f64x8::splat(4.0)).sqrt().sqrt());
            let t86 = t85 * t85;
            let t87 = t86 * t85;
            let t88 = ((t56).sqrt().sqrt());
            let t92 = t27 + f64x8::splat(5.0) / f64x8::splat(8.0) * t87 * t88 * t56;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t3 * f64x8::splat(M_PI);
            let t96 = f64x8::splat(1.0) / t29 / t94;
            let t98 = f64x8::splat(12.0) * t1;
            let t99 = f64x8::splat(7.0) / f64x8::splat(6.0) * t3 - t98 - f64x8::splat(1.0);
            let t100 = t96 * t99;
            let t101 = t14 * t46;
            let t105 = f64x8::splat(1.0) + t101 * t19 * t21 / f64x8::splat(3.0);
            let t106 = (simd::ln(t105));
            let t110 = -t64 * t100 * t106 / f64x8::splat(36.0) - f64x8::splat(0.01);
            let t111 = t93 * t110;
            let t114 = t83 * t84 * t111 / f64x8::splat(4.0);
            let t119 = (simd::exp(-f64x8::splat(4.0) * (-f64x8::splat(0.1412623711751798) + t6) * t8 * t3));
            let t120 = f64x8::splat(M_CBRT2);
            let t128 = t119 / f64x8::splat(2.0);
            let t129 = f64x8::splat(2.0) * (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t33 * t34 * t120) * t8 * t3 + t128;
            let t133 = (-f64x8::splat(2.0) * t129 * t28 + t119) * t14;
            let t137 = t120 * t120;
            let t142 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t64 * t65 * t137 * t8 + t128;
            let t146 = (-f64x8::splat(2.0) * t142 * t28 + t119) * t13;
            let t149 = f64x8::splat(1.0) + t133 * t48 / f64x8::splat(3.0) - f64x8::splat(236.87050562614462) * t61 + t146 * t76 / f64x8::splat(3.0);
            let t150 = (simd::ln(t149));
            let t155 = t137 * t63;
            let t157 = f64x8::splat(13.0) / f64x8::splat(12.0) * t3 - t98 + f64x8::splat(1.0) / f64x8::splat(2.0);
            let t158 = t96 * t157;
            let t160 = t155 * t158 * t106;
            let t163 = -t5 * t150 / f64x8::splat(4.0) - t53 * t84 * t93 * t160 / f64x8::splat(144.0) + t82 - t114;
            let t164 = v_rho0 - v_rho1;
            let t165 = f64x8::splat(1.0) / t20;
            let t166 = t164 * t165;
            let t167 = f64x8::splat(1.0) + t166;
            let t168 = (t167).simd_le(zeta_threshold);
            let t169 = (simd::cbrt(zeta_threshold));
            let t170 = t169 * zeta_threshold;
            let t171 = (simd::cbrt(t167));
            let t173 = ((t168).select(t170, t171 * t167));
            let t174 = f64x8::splat(1.0) - t166;
            let t175 = (t174).simd_le(zeta_threshold);
            let t176 = (simd::cbrt(t174));
            let t178 = ((t175).select(t170, t176 * t174));
            let t179 = t173 + t178 - f64x8::splat(2.0);
            let t183 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t120 - f64x8::splat(2.0));
            let t184 = t163 * t179 * t183;
            let tzk0 = -t82 + t114 + t184;
            acc_zk = tzk0;
            let t186 = f64x8::splat(1.0) / t21 / t20;
            let t187 = t186 * t27;
            let t191 = t47 * t23;
            let t194 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t195 = t194 * t194;
            let t196 = t195 * t195;
            let t197 = t196 * t194;
            let t198 = t18 * t197;
            let t200 = f64x8::splat(1.0) / t22 / t20;
            let t201 = t200 * t27;
            let t202 = t8 * t59;
            let t204 = t198 * t201 * t202;
            let t206 = t51 * t194;
            let t207 = f64x8::splat(4.0) * t25;
            let t209 = f64x8::splat(1.0) / t57 / t207;
            let t210 = t209 * t13;
            let t213 = t206 * t210 * t16 * t186;
            let t215 = t165 * t27;
            let t218 = t75 * t54;
            let t221 = t83 * t187 * t41 / f64x8::splat(30000.0) + t45 * t191 / f64x8::splat(9.0) + f64x8::splat(0.0019739208802178718) * t204 - f64x8::splat(236.87050562614462) * t213 + t215 * t69 / f64x8::splat(7500.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t218;
            let t222 = f64x8::splat(1.0) / t79;
            let t224 = t5 * t221 * t222;
            let t225 = t224 / f64x8::splat(2.0);
            let t227 = t83 * t187 * t111;
            let t228 = t227 / f64x8::splat(12.0);
            let t229 = t20 * t20;
            let t230 = f64x8::splat(1.0) / t229;
            let t231 = t15 * t230;
            let t232 = t27 * t93;
            let t233 = t232 * t110;
            let t234 = t231 * t233;
            let t235 = t234 / f64x8::splat(20000.0);
            let t236 = t92 * t92;
            let t237 = f64x8::splat(1.0) / t236;
            let t238 = t27 * t237;
            let t239 = t19 * t200;
            let t243 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t244 = t243 * t243;
            let t245 = t244 * t244;
            let t246 = t245 * t243;
            let t247 = t246 * t88;
            let t248 = t53 * t186;
            let t251 = t18 * t239 * t27 / f64x8::splat(60000.0) - f64x8::splat(25.0) / f64x8::splat(24.0) * t247 * t248;
            let t252 = t110 * t251;
            let t253 = t238 * t252;
            let t254 = t56 * t253;
            let t255 = t254 / f64x8::splat(4.0);
            let t256 = t19 * t165;
            let t258 = t63 * t96;
            let t259 = f64x8::splat(1.0) / t105;
            let t260 = t99 * t259;
            let t261 = t258 * t260;
            let t262 = t256 * t232 * t261;
            let t263 = t262 / f64x8::splat(108.0);
            let t275 = t83 * t187 * t129 / f64x8::splat(30000.0) + t133 * t191 / f64x8::splat(9.0) + f64x8::splat(0.0039478417604357436) * t204 - f64x8::splat(473.74101125228924) * t213 + t215 * t142 / f64x8::splat(7500.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t146 * t218;
            let t276 = f64x8::splat(1.0) / t149;
            let t284 = t3 * t3;
            let t286 = f64x8::splat(1.0) / t29 / t284;
            let t287 = t286 * t230;
            let t288 = t19 * t27;
            let t290 = t93 * t137;
            let t291 = t63 * t157;
            let t292 = t291 * t106;
            let t293 = t290 * t292;
            let t297 = t53 * t84 * t237;
            let t298 = t155 * t96;
            let t299 = t157 * t106;
            let t300 = t299 * t251;
            let t301 = t298 * t300;
            let t307 = t258 * t157 * t19 * t259;
            let t310 = -t5 * t275 * t276 / f64x8::splat(4.0) + t53 * t187 * t93 * t160 / f64x8::splat(432.0) - t287 * t288 * t293 / f64x8::splat(2880000.0) + t297 * t301 / f64x8::splat(144.0) - t215 * t290 * t307 / f64x8::splat(432.0) + t225 + t228 - t235 + t255 + t263;
            let t312 = t310 * t179 * t183;
            let t313 = t164 * t230;
            let t314 = t165 - t313;
            let t317 = ((t168).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t314));
            let t318 = -t314;
            let t321 = ((t175).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t318));
            let t322 = t317 + t321;
            let t324 = t163 * t322 * t183;
            let tvrho0 = -t82 + t114 + t184 + t20 * (-t225 - t228 + t235 - t255 - t263 + t312 + t324);
            acc_vrho_0 = tvrho0;
            let t327 = -t165 - t313;
            let t330 = ((t168).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t327));
            let t331 = -t327;
            let t334 = ((t175).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t331));
            let t335 = t330 + t334;
            let t337 = t163 * t335 * t183;
            let tvrho1 = -t82 + t114 + t184 + t20 * (-t225 - t228 + t235 - t255 - t263 + t312 + t337);
            acc_vrho_1 = tvrho1;
            let t340 = t227 / f64x8::splat(6.0);
            let t341 = t234 / f64x8::splat(10000.0);
            let t342 = t254 / f64x8::splat(2.0);
            let t343 = t262 / f64x8::splat(54.0);
            let t344 = f64x8::splat(2.0) * t312;
            let t347 = f64x8::splat(1.0) / t21 / t229;
            let t348 = t347 * t27;
            let t352 = t229 * t20;
            let t353 = f64x8::splat(1.0) / t352;
            let t354 = t15 * t353;
            let t355 = t27 * t41;
            let t358 = t47 * t200;
            let t362 = f64x8::splat(1.0) / t22 / t229;
            let t363 = t362 * t27;
            let t365 = t198 * t363 * t202;
            let t367 = t16 * t15;
            let t368 = t13 * t367;
            let t369 = t368 * t194;
            let t371 = f64x8::splat(1.0) / t21 / t352;
            let t372 = t371 * t27;
            let t374 = t369 * t372 * t202;
            let t376 = t15 * t52;
            let t378 = t27 * t8;
            let t379 = t378 * t209;
            let t380 = t376 * t353 * t379;
            let t382 = t51 * t197;
            let t386 = f64x8::splat(1.0) / t57 / t15 / t165 / f64x8::splat(48.0);
            let t387 = t386 * t14;
            let t390 = t382 * t387 * t17 * t362;
            let t394 = t206 * t210 * t16 * t347;
            let t396 = t230 * t27;
            let t400 = t362 * t14 * t17;
            let t401 = t288 * t69;
            let t404 = t75 * t186;
            let t407 = -t83 * t348 * t41 / f64x8::splat(30000.0) + t354 * t355 / f64x8::splat(150000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t45 * t358 - f64x8::splat(0.003289868133696453) * t365 + f64x8::splat(3.9478417604357434e-07) * t374 + f64x8::splat(0.02368705056261446) * t380 - f64x8::splat(197.39208802178717) * t390 + f64x8::splat(315.82734083485946) * t394 - t396 * t69 / f64x8::splat(22500.0) + t400 * t401 / f64x8::splat(450000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t73 * t404;
            let t409 = t5 * t407 * t222;
            let t410 = t409 / f64x8::splat(2.0);
            let t411 = t221 * t221;
            let t412 = t79 * t79;
            let t413 = f64x8::splat(1.0) / t412;
            let t415 = t5 * t411 * t413;
            let t416 = t415 / f64x8::splat(2.0);
            let t418 = t83 * t348 * t111;
            let t419 = t418 / f64x8::splat(9.0);
            let t420 = t354 * t233;
            let t421 = f64x8::splat(7.0) / f64x8::splat(60000.0) * t420;
            let t422 = t34 * t186;
            let t423 = t53 * t422;
            let t424 = t423 * t253;
            let t425 = t424 / f64x8::splat(6.0);
            let t426 = t19 * t230;
            let t428 = t426 * t232 * t261;
            let t429 = t428 / f64x8::splat(81.0);
            let t431 = f64x8::splat(1.0) / t22 / t352;
            let t433 = t15 * t431 * t18;
            let t434 = t288 * t111;
            let t435 = t433 * t434;
            let t436 = t435 / f64x8::splat(1200000000.0);
            let t437 = t231 * t27;
            let t438 = t237 * t110;
            let t439 = t438 * t251;
            let t440 = t437 * t439;
            let t441 = t440 / f64x8::splat(10000.0);
            let t442 = t286 * t362;
            let t443 = t232 * t63;
            let t445 = t34 * t99;
            let t446 = t101 * t259;
            let t447 = t445 * t446;
            let t448 = t442 * t443 * t447;
            let t449 = t448 / f64x8::splat(6480000.0);
            let t451 = f64x8::splat(1.0) / t236 / t92;
            let t452 = t27 * t451;
            let t453 = t251 * t251;
            let t454 = t110 * t453;
            let t455 = t452 * t454;
            let t456 = t56 * t455;
            let t457 = t456 / f64x8::splat(2.0);
            let t458 = t256 * t238;
            let t459 = t260 * t251;
            let t460 = t258 * t459;
            let t461 = t458 * t460;
            let t462 = t461 / f64x8::splat(54.0);
            let t467 = t34 * t371;
            let t471 = t88 * t88;
            let t472 = t471 * t88;
            let t474 = t243 / t472;
            let t480 = -t18 * t19 * t362 * t27 / f64x8::splat(36000.0) + t368 * t467 * t27 / f64x8::splat(1200000000.0) + f64x8::splat(25.0) / f64x8::splat(72.0) * t474 * t400 + f64x8::splat(25.0) / f64x8::splat(18.0) * t247 * t53 * t347;
            let t482 = t238 * t110 * t480;
            let t483 = t56 * t482;
            let t484 = t483 / f64x8::splat(4.0);
            let t485 = t34 * t362;
            let t486 = t18 * t27;
            let t487 = t485 * t486;
            let t488 = t93 * t63;
            let t490 = t488 * t100 * t259;
            let t491 = t487 * t490;
            let t492 = t491 / f64x8::splat(6480000.0);
            let t493 = t34 * t200;
            let t495 = t105 * t105;
            let t496 = f64x8::splat(1.0) / t495;
            let t498 = t496 * t14 * t46;
            let t499 = t100 * t498;
            let t500 = t493 * t443 * t499;
            let t501 = t500 / f64x8::splat(972.0);
            let t505 = t286 * t353;
            let t509 = t288 * t237;
            let t510 = t287 * t509;
            let t511 = t155 * t300;
            let t515 = t237 * t137 * t63;
            let t516 = t215 * t515;
            let t517 = t19 * t259;
            let t519 = t158 * t517 * t251;
            let t526 = t441 - t419 + t396 * t290 * t307 / f64x8::splat(324.0) - t462 + f64x8::splat(7.0) / f64x8::splat(8640000.0) * t505 * t288 * t293 - t425 - t429 - t436 - t457 + t484 + t510 * t511 / f64x8::splat(1440000.0) + t516 * t519 / f64x8::splat(216.0) - t53 * t348 * t93 * t160 / f64x8::splat(324.0) + t449;
            let t528 = t53 * t84 * t451;
            let t529 = t299 * t453;
            let t530 = t298 * t529;
            let t533 = t34 * t27;
            let t534 = t533 * t93;
            let t536 = t155 * t157;
            let t537 = t536 * t446;
            let t540 = t299 * t480;
            let t541 = t298 * t540;
            let t544 = t290 * t63;
            let t545 = t158 * t259;
            let t546 = t544 * t545;
            let t551 = t158 * t34 * t498;
            let t555 = t53 * t187 * t237;
            let t558 = t286 * t431;
            let t560 = t34 * t14 * t17;
            let t562 = t232 * t137;
            let t563 = t562 * t292;
            let t566 = t275 * t275;
            let t567 = t149 * t149;
            let t568 = f64x8::splat(1.0) / t567;
            let t575 = t27 * t129;
            let t587 = t288 * t142;
            let t592 = -t83 * t348 * t129 / f64x8::splat(30000.0) + t354 * t575 / f64x8::splat(150000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t133 * t358 - f64x8::splat(0.006579736267392906) * t365 + f64x8::splat(7.895683520871487e-07) * t374 + f64x8::splat(0.04737410112522892) * t380 - f64x8::splat(394.78417604357435) * t390 + f64x8::splat(631.6546816697189) * t394 - t396 * t142 / f64x8::splat(22500.0) + t400 * t587 / f64x8::splat(450000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t146 * t404;
            let t596 = t492 - t501 - t528 * t530 / f64x8::splat(72.0) - t442 * t534 * t537 / f64x8::splat(25920000.0) + t297 * t541 / f64x8::splat(144.0) - t487 * t546 / f64x8::splat(25920000.0) + t201 * t544 * t551 / f64x8::splat(3888.0) - t555 * t301 / f64x8::splat(216.0) - t558 * t560 * t563 / f64x8::splat(172800000000.0) + t421 + t410 - t416 + t5 * t566 * t568 / f64x8::splat(4.0) - t5 * t592 * t276 / f64x8::splat(4.0);
            let t597 = t526 + t596;
            let t599 = t597 * t179 * t183;
            let t601 = t310 * t322 * t183;
            let t602 = f64x8::splat(2.0) * t601;
            let t603 = t171 * t171;
            let t604 = f64x8::splat(1.0) / t603;
            let t605 = t314 * t314;
            let t608 = t164 * t353;
            let t610 = -f64x8::splat(2.0) * t230 + f64x8::splat(2.0) * t608;
            let t614 = ((t168).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t604 * t605 + f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t610));
            let t615 = t176 * t176;
            let t616 = f64x8::splat(1.0) / t615;
            let t617 = t318 * t318;
            let t620 = -t610;
            let t624 = ((t175).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t616 * t617 + f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t620));
            let t625 = t614 + t624;
            let t627 = t163 * t625 * t183;
            let t628 = -t410 + t416 + t419 - t421 + t425 + t429 + t436 - t441 - t449 + t457 + t462 - t484 - t492 + t501 + t599 + t602 + t627;
            let tv2rho20 = t20 * t628 - t224 + f64x8::splat(2.0) * t324 - t340 + t341 - t342 - t343 + t344;
            acc_v2rho2_0 = tv2rho20;
            let t631 = t310 * t335 * t183;
            let t632 = t604 * t327;
            let t635 = t171 * t164;
            let t639 = ((t168).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t632 * t314 + f64x8::splat(8.0) / f64x8::splat(3.0) * t635 * t353));
            let t640 = t616 * t331;
            let t643 = t176 * t164;
            let t647 = ((t175).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t640 * t318 - f64x8::splat(8.0) / f64x8::splat(3.0) * t643 * t353));
            let t648 = t639 + t647;
            let t650 = t163 * t648 * t183;
            let t651 = -t410 + t416 + t419 - t421 + t425 + t429 + t436 - t441 - t449 + t457 + t462 - t484 - t492 + t501 + t599 + t601 + t631 + t650;
            let tv2rho21 = t20 * t651 - t224 + t324 + t337 - t340 + t341 - t342 - t343 + t344;
            acc_v2rho2_1 = tv2rho21;
            let t654 = f64x8::splat(2.0) * t631;
            let t655 = t327 * t327;
            let t659 = f64x8::splat(2.0) * t230 + f64x8::splat(2.0) * t608;
            let t663 = ((t168).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t604 * t655 + f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t659));
            let t664 = t331 * t331;
            let t667 = -t659;
            let t671 = ((t175).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t616 * t664 + f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t667));
            let t672 = t663 + t671;
            let t674 = t163 * t672 * t183;
            let t675 = -t410 + t416 + t419 - t421 + t425 + t429 + t436 - t441 - t449 + t457 + t462 - t484 - t492 + t501 + t599 + t654 + t674;
            let tv2rho22 = t20 * t675 - t224 + f64x8::splat(2.0) * t337 - t340 + t341 - t342 - t343 + t344;
            acc_v2rho2_2 = tv2rho22;
            let t679 = t5 * t407 * t413 * t221;
            let t680 = f64x8::splat(3.0) / f64x8::splat(2.0) * t679;
            let t681 = t229 * t229;
            let t682 = f64x8::splat(1.0) / t681;
            let t683 = t15 * t682;
            let t684 = t683 * t233;
            let t685 = f64x8::splat(67.0) / f64x8::splat(180000.0) * t684;
            let t686 = t451 * t110;
            let t687 = t686 * t453;
            let t688 = t437 * t687;
            let t689 = f64x8::splat(3.0) / f64x8::splat(10000.0) * t688;
            let t690 = t438 * t480;
            let t691 = t437 * t690;
            let t692 = f64x8::splat(3.0) / f64x8::splat(20000.0) * t691;
            let t693 = t354 * t27;
            let t694 = t693 * t439;
            let t695 = f64x8::splat(7.0) / f64x8::splat(20000.0) * t694;
            let t697 = t83 * t372 * t111;
            let t698 = f64x8::splat(7.0) / f64x8::splat(27.0) * t697;
            let t700 = t597 * t322 * t183;
            let t701 = f64x8::splat(3.0) * t700;
            let t702 = t275 * t568;
            let t706 = t286 * t682;
            let t710 = t353 * t27;
            let t711 = t710 * t290;
            let t715 = f64x8::splat(1.0) / t21 / t681;
            let t716 = t715 * t13;
            let t717 = t367 * t27;
            let t719 = t716 * t717 * t490;
            let t720 = t719 / f64x8::splat(32400000000.0);
            let t721 = t371 * t13;
            let t722 = t16 * t27;
            let t724 = t100 * t496;
            let t725 = t488 * t724;
            let t726 = t721 * t722 * t725;
            let t727 = t726 / f64x8::splat(2430000.0);
            let t730 = f64x8::splat(1.0) / t495 / t105;
            let t732 = t730 * t13 * t74;
            let t733 = t100 * t732;
            let t734 = t348 * t488 * t733;
            let t735 = f64x8::splat(2.0) / f64x8::splat(729.0) * t734;
            let t736 = t288 * t439;
            let t737 = t433 * t736;
            let t738 = t737 / f64x8::splat(400000000.0);
            let t739 = t423 * t482;
            let t740 = t739 / f64x8::splat(4.0);
            let t741 = t681 * t20;
            let t743 = f64x8::splat(1.0) / t21 / t741;
            let t744 = t15 * t743;
            let t745 = t744 * t368;
            let t746 = t533 * t111;
            let t747 = t745 * t746;
            let t748 = t747 / f64x8::splat(24000000000000.0);
            let t757 = f64x8::splat(1.0) / t741;
            let t763 = t87 / t472 / t56;
            let t766 = t18 * t431;
            let t772 = t18 * t19 * t431 * t27 / f64x8::splat(13500.0) - t368 * t34 * t715 * t27 / f64x8::splat(240000000.0) + t4 * t757 * t27 / f64x8::splat(6000000000000.0) + f64x8::splat(25.0) / f64x8::splat(96.0) * t763 * t683 - f64x8::splat(25.0) / f64x8::splat(18.0) * t474 * t766 - f64x8::splat(175.0) / f64x8::splat(54.0) * t247 * t53 * t371;
            let t774 = t238 * t110 * t772;
            let t775 = t56 * t774;
            let t776 = t775 / f64x8::splat(4.0);
            let t777 = f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t702 * t592 - t680 - t685 - t689 + t692 - t695 + t698 - f64x8::splat(67.0) / f64x8::splat(25920000.0) * t706 * t288 * t293 - f64x8::splat(7.0) / f64x8::splat(972.0) * t711 * t307 + t720 - t727 + t735 + t738 - t740 - t748 + t776;
            let t779 = t53 * t34 * t347;
            let t780 = t779 * t253;
            let t781 = t780 / f64x8::splat(3.0);
            let t782 = t236 * t236;
            let t783 = f64x8::splat(1.0) / t782;
            let t784 = t27 * t783;
            let t785 = t453 * t251;
            let t787 = t784 * t110 * t785;
            let t788 = t56 * t787;
            let t789 = f64x8::splat(3.0) / f64x8::splat(2.0) * t788;
            let t790 = t423 * t455;
            let t791 = t790 / f64x8::splat(2.0);
            let t792 = t19 * t353;
            let t794 = t792 * t232 * t261;
            let t795 = f64x8::splat(7.0) / f64x8::splat(243.0) * t794;
            let t797 = f64x8::splat(1.0) / t22 / t681;
            let t798 = t15 * t797;
            let t799 = t798 * t18;
            let t800 = t799 * t434;
            let t801 = t800 / f64x8::splat(200000000.0);
            let t802 = t286 * t715;
            let t803 = t53 * t27;
            let t804 = t802 * t803;
            let t806 = t290 * t291 * t259;
            let t809 = t155 * t540;
            let t813 = t158 * t517 * t480;
            let t816 = t288 * t451;
            let t817 = t287 * t816;
            let t818 = t155 * t529;
            let t822 = t451 * t137 * t63;
            let t823 = t215 * t822;
            let t825 = t158 * t517 * t453;
            let t828 = t34 * t431;
            let t829 = t828 * t486;
            let t830 = t829 * t490;
            let t831 = t830 / f64x8::splat(1620000.0);
            let t833 = t485 * t443 * t499;
            let t834 = t833 / f64x8::splat(324.0);
            let t836 = t558 * t443 * t447;
            let t837 = t836 / f64x8::splat(1296000.0);
            let t838 = t396 * t515;
            let t841 = t505 * t509;
            let t845 = t53 * t372 * t93;
            let t848 = t781 + t789 + t791 + t795 + t801 - t804 * t806 / f64x8::splat(64800000000.0) + t510 * t809 / f64x8::splat(960000.0) + t516 * t813 / f64x8::splat(144.0) - t817 * t818 / f64x8::splat(480000.0) - t823 * t825 / f64x8::splat(72.0) - t831 + t834 - t837 - t838 * t519 / f64x8::splat(108.0) - f64x8::splat(7.0) / f64x8::splat(2880000.0) * t841 * t511 + f64x8::splat(7.0) / f64x8::splat(972.0) * t845 * t160;
            let t850 = t286 * t371;
            let t852 = t13 * t74;
            let t853 = t852 * t496;
            let t854 = t291 * t853;
            let t857 = t717 * t93;
            let t859 = t155 * t545;
            let t863 = t155 * t158 * t496;
            let t867 = t158 * t732;
            let t870 = t286 * t743;
            let t871 = t368 * t27;
            let t872 = t870 * t871;
            let t875 = t237 * t63;
            let t876 = t875 * t96;
            let t877 = t876 * t459;
            let t878 = t487 * t877;
            let t879 = t878 / f64x8::splat(2160000.0);
            let t880 = t238 * t63;
            let t881 = t493 * t880;
            let t883 = t251 * t14 * t46;
            let t884 = t724 * t883;
            let t885 = t881 * t884;
            let t886 = t885 / f64x8::splat(324.0);
            let t887 = t442 * t880;
            let t888 = t445 * t14;
            let t889 = t46 * t259;
            let t890 = t889 * t251;
            let t891 = t888 * t890;
            let t892 = t887 * t891;
            let t893 = t892 / f64x8::splat(2160000.0);
            let t894 = t299 * t772;
            let t895 = t298 * t894;
            let t899 = t53 * t348 * t237;
            let t902 = t286 * t797;
            let t920 = t850 * t562 * t854 / f64x8::splat(19440000.0) - t716 * t857 * t859 / f64x8::splat(129600000000.0) + t845 * t863 / f64x8::splat(9720000.0) - t348 * t544 * t867 / f64x8::splat(1458.0) - t872 * t293 / f64x8::splat(864000000000000.0) - t879 + t886 - t893 + t297 * t895 / f64x8::splat(144.0) + t899 * t301 / f64x8::splat(108.0) + t902 * t560 * t563 / f64x8::splat(28800000000.0) - t555 * t541 / f64x8::splat(144.0) + t829 * t546 / f64x8::splat(6480000.0) - t363 * t544 * t551 / f64x8::splat(1296.0) + t558 * t534 * t537 / f64x8::splat(5184000.0) + t53 * t187 * t451 * t530 / f64x8::splat(72.0);
            let t923 = t299 * t785;
            let t924 = t298 * t923;
            let t928 = t63 * t99;
            let t930 = t232 * t928 * t259;
            let t931 = t802 * t53 * t930;
            let t932 = t931 / f64x8::splat(16200000000.0);
            let t934 = t928 * t853;
            let t935 = t850 * t232 * t934;
            let t936 = t935 / f64x8::splat(4860000.0);
            let t938 = t452 * t252 * t480;
            let t939 = t56 * t938;
            let t940 = f64x8::splat(3.0) / f64x8::splat(2.0) * t939;
            let t942 = t260 * t453;
            let t943 = t258 * t942;
            let t944 = t256 * t452 * t943;
            let t945 = t944 / f64x8::splat(18.0);
            let t946 = t260 * t480;
            let t947 = t258 * t946;
            let t948 = t458 * t947;
            let t949 = t948 / f64x8::splat(36.0);
            let t950 = t426 * t238;
            let t951 = t950 * t460;
            let t952 = t951 / f64x8::splat(27.0);
            let t955 = f64x8::splat(1.0) / t412 / t79;
            let t957 = t5 * t411 * t221 * t955;
            let t960 = f64x8::splat(1.0) / t567 / t149;
            let t967 = t798 * t14;
            let t968 = t17 * t19;
            let t969 = t968 * t575;
            let t972 = t716 * t367;
            let t973 = t533 * t142;
            let t976 = t75 * t347;
            let t979 = t47 * t362;
            let t984 = t51 * t52;
            let t988 = f64x8::splat(1.0) / t57 / t368 / t422 / f64x8::splat(48.0);
            let t989 = t988 * t15;
            let t991 = t984 * t989 * t682;
            let t994 = t376 * t682 * t379;
            let t996 = t4 * t52;
            let t998 = t378 * t59;
            let t999 = t996 * t757 * t998;
            let t1001 = t715 * t27;
            let t1003 = t369 * t1001 * t202;
            let t1006 = t14 * t17 * t15;
            let t1007 = t1006 * t197;
            let t1008 = t797 * t27;
            let t1009 = t8 * t209;
            let t1011 = t1007 * t1008 * t1009;
            let t1013 = t8 * t386;
            let t1015 = t369 * t1001 * t1013;
            let t1019 = t206 * t210 * t16 * t371;
            let t1021 = t431 * t27;
            let t1023 = t198 * t1021 * t202;
            let t1025 = t15 * t197;
            let t1028 = t17 * t27;
            let t1029 = t1028 * t1009;
            let t1030 = t1025 * t797 * t14 * t1029;
            let t1032 = t15 * t194;
            let t1034 = t1013 * t53;
            let t1035 = t1032 * t1001 * t1034;
            let t1039 = t382 * t387 * t17 * t431;
            let t1045 = f64x8::splat(19.0) / f64x8::splat(270000.0) * t83 * t372 * t129 + t967 * t969 / f64x8::splat(9000000000000.0) + t972 * t973 / f64x8::splat(9000000000000.0) + f64x8::splat(8.0) / f64x8::splat(81.0) * t146 * t976 + f64x8::splat(10.0) / f64x8::splat(81.0) * t133 * t979 - t766 * t587 / f64x8::splat(150000000.0) - f64x8::splat(5526.978464610041) * t991 - f64x8::splat(0.21318345506353015) * t994 + f64x8::splat(3.947841760435743e-11) * t999 - f64x8::splat(3.947841760435744e-06) * t1003 + f64x8::splat(3.9478417604357434e-07) * t1011 + f64x8::splat(0.07895683520871487) * t1015 - f64x8::splat(1473.860923896011) * t1019 + f64x8::splat(0.017545963379714414) * t1023 + f64x8::splat(7.895683520871487e-07) * t1030 + f64x8::splat(0.15791367041742974) * t1035 + f64x8::splat(1579.1367041742974) * t1039 - t683 * t575 / f64x8::splat(37500000.0) + t710 * t142 / f64x8::splat(16875.0);
            let t1049 = t558 * t34;
            let t1050 = t1049 * t486;
            let t1051 = t515 * t300;
            let t1054 = t442 * t34;
            let t1055 = t238 * t137;
            let t1056 = t1054 * t1055;
            let t1057 = t291 * t14;
            let t1058 = t1057 * t890;
            let t1061 = t533 * t237;
            let t1062 = t400 * t1061;
            let t1063 = t157 * t259;
            let t1064 = t1063 * t251;
            let t1065 = t298 * t1064;
            let t1069 = t201 * t237 * t298;
            let t1071 = t157 * t34 * t496;
            let t1072 = t1071 * t883;
            let t1075 = t53 * t54;
            let t1076 = t452 * t137;
            let t1077 = t1075 * t1076;
            let t1078 = t258 * t157;
            let t1080 = t106 * t251 * t480;
            let t1081 = t1078 * t1080;
            let t1087 = t533 * t69;
            let t1102 = t968 * t355;
            let t1114 = -f64x8::splat(2763.4892323050203) * t991 - t766 * t401 / f64x8::splat(150000000.0) + t972 * t1087 / f64x8::splat(9000000000000.0) + f64x8::splat(8.0) / f64x8::splat(81.0) * t73 * t976 + f64x8::splat(10.0) / f64x8::splat(81.0) * t45 * t979 - f64x8::splat(0.10659172753176507) * t994 + f64x8::splat(1.9739208802178716e-11) * t999 - f64x8::splat(1.973920880217872e-06) * t1003 + f64x8::splat(1.9739208802178717e-07) * t1011 + f64x8::splat(0.039478417604357434) * t1015 + f64x8::splat(19.0) / f64x8::splat(270000.0) * t83 * t372 * t41 + t967 * t1102 / f64x8::splat(9000000000000.0) - f64x8::splat(736.9304619480055) * t1019 + f64x8::splat(0.008772981689857207) * t1023 + f64x8::splat(3.9478417604357434e-07) * t1030 + f64x8::splat(0.07895683520871487) * t1035 + f64x8::splat(789.5683520871487) * t1039 - t683 * t355 / f64x8::splat(37500000.0) + t710 * t69 / f64x8::splat(16875.0);
            let t1116 = t5 * t1114 * t222;
            let t1117 = t1116 / f64x8::splat(2.0);
            let t1118 = t53 * t84 * t783 * t924 / f64x8::splat(24.0) + t932 - t936 - t940 + t945 - t949 + t952 + t957 - t5 * t566 * t275 * t960 / f64x8::splat(2.0) - t5 * t1045 * t276 / f64x8::splat(4.0) + t1050 * t1051 / f64x8::splat(57600000000.0) + t1056 * t1058 / f64x8::splat(8640000.0) + t1062 * t1065 / f64x8::splat(8640000.0) - t1069 * t1072 / f64x8::splat(1296.0) - t1077 * t1081 / f64x8::splat(24.0) + t1117;
            let t1120 = t777 + t848 + t920 + t1118;
            let t1122 = t1120 * t179 * t183;
            let t1124 = t310 * t625 * t183;
            let t1127 = f64x8::splat(1.0) / t603 / t167;
            let t1128 = t605 * t314;
            let t1131 = t604 * t314;
            let t1134 = t164 * t682;
            let t1136 = f64x8::splat(6.0) * t353 - f64x8::splat(6.0) * t1134;
            let t1140 = ((t168).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1127 * t1128 + f64x8::splat(4.0) / f64x8::splat(3.0) * t1131 * t610 + f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t1136));
            let t1142 = f64x8::splat(1.0) / t615 / t174;
            let t1143 = t617 * t318;
            let t1146 = t616 * t318;
            let t1149 = -t1136;
            let t1153 = ((t175).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1142 * t1143 + f64x8::splat(4.0) / f64x8::splat(3.0) * t1146 * t620 + f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t1149));
            let t1154 = t1140 + t1153;
            let t1156 = t163 * t1154 * t183;
            let t1157 = t680 + t685 + t689 - t692 + t695 - t698 + t701 + t1122 - t720 + t727 - t735 - t738 + f64x8::splat(3.0) * t1124 + t1156 + t740 + t748 - t776 - t781;
            let t1158 = -t789 - t791 - t795 - t801 + t831 - t834 + t837 + t879 - t886 + t893 - t932 + t936 + t940 - t945 + t949 - t952 - t957 - t1117;
            let t1161 = t500 / f64x8::splat(324.0);
            let t1162 = t448 / f64x8::splat(2160000.0);
            let t1163 = t491 / f64x8::splat(2160000.0);
            let t1164 = t418 / f64x8::splat(3.0);
            let t1165 = t424 / f64x8::splat(2.0);
            let t1166 = t428 / f64x8::splat(27.0);
            let t1167 = t435 / f64x8::splat(400000000.0);
            let t1168 = f64x8::splat(3.0) / f64x8::splat(4.0) * t483;
            let t1169 = f64x8::splat(7.0) / f64x8::splat(20000.0) * t420;
            let t1170 = f64x8::splat(3.0) / f64x8::splat(2.0) * t415;
            let t1171 = f64x8::splat(3.0) / f64x8::splat(2.0) * t456;
            let t1172 = t461 / f64x8::splat(18.0);
            let t1173 = f64x8::splat(3.0) / f64x8::splat(10000.0) * t440;
            let t1174 = f64x8::splat(3.0) * t599;
            let t1177 = f64x8::splat(3.0) / f64x8::splat(2.0) * t409;
            let tv3rho30 = t20 * (t1157 + t1158) + t1161 - t1162 - t1163 + t1164 + t1165 + t1166 + t1167 - t1168 - t1169 + t1170 + t1171 + t1172 - t1173 + t1174 + f64x8::splat(6.0) * t601 + f64x8::splat(3.0) * t627 - t1177;
            acc_v3rho3_0 = tv3rho30;
            let t1179 = t310 * t648 * t183;
            let t1180 = f64x8::splat(2.0) * t1179;
            let t1182 = t597 * t335 * t183;
            let t1183 = t1127 * t327;
            let t1186 = t604 * t164;
            let t1197 = ((t168).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1183 * t605 + f64x8::splat(16.0) / f64x8::splat(9.0) * t1186 * t353 * t314 + f64x8::splat(4.0) / f64x8::splat(9.0) * t632 * t610 + f64x8::splat(8.0) / f64x8::splat(3.0) * t171 * t353 - f64x8::splat(8.0) * t635 * t682));
            let t1198 = t1142 * t331;
            let t1201 = t616 * t164;
            let t1212 = ((t175).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1198 * t617 - f64x8::splat(16.0) / f64x8::splat(9.0) * t1201 * t353 * t318 + f64x8::splat(4.0) / f64x8::splat(9.0) * t640 * t620 - f64x8::splat(8.0) / f64x8::splat(3.0) * t176 * t353 + f64x8::splat(8.0) * t643 * t682));
            let t1213 = t1197 + t1212;
            let t1215 = t163 * t1213 * t183;
            let t1217 = t680 + t685 + t689 - t692 + t695 - t698 + t1180 + t1182 + t1215 + f64x8::splat(2.0) * t700 + t1122 - t720 + t727 - t735 - t738 + t1124 + t740 + t748 - t776;
            let t1218 = -t781 - t789 - t791 - t795 - t801 + t831 - t834 + t837 + t879 - t886 + t893 - t932 + t936 + t940 - t945 + t949 - t952 - t957 - t1117;
            let t1222 = f64x8::splat(2.0) * t650;
            let tv3rho31 = t20 * (t1217 + t1218) + t1161 - t1162 - t1163 + t1164 + t1165 + t1166 + t1167 - t1168 - t1169 + t1170 + t1171 + t1172 - t1173 + t1174 + f64x8::splat(4.0) * t601 + t627 - t1177 + t654 + t1222;
            acc_v3rho3_1 = tv3rho31;
            let t1224 = t680 + t685 + t689 - t692 + t695 - t698 + t1180 + f64x8::splat(2.0) * t1182 + t700 + t1122 - t720 + t727 - t735 - t738 + t740 + t748 - t776 - t781 - t789;
            let t1226 = t310 * t672 * t183;
            let t1227 = t1127 * t655;
            let t1232 = t604 * t659;
            let t1237 = -f64x8::splat(2.0) * t353 - f64x8::splat(6.0) * t1134;
            let t1241 = ((t168).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1227 * t314 + f64x8::splat(16.0) / f64x8::splat(9.0) * t632 * t608 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1232 * t314 + f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t1237));
            let t1242 = t1142 * t664;
            let t1247 = t616 * t667;
            let t1250 = -t1237;
            let t1254 = ((t175).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1242 * t318 - f64x8::splat(16.0) / f64x8::splat(9.0) * t640 * t608 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1247 * t318 + f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t1250));
            let t1255 = t1241 + t1254;
            let t1257 = t163 * t1255 * t183;
            let t1258 = -t791 - t795 - t801 + t831 - t834 + t837 + t879 - t886 + t893 - t932 + t936 + t940 - t945 + t949 - t952 + t1226 + t1257 - t957 - t1117;
            let tv3rho32 = t20 * (t1224 + t1258) + t1161 - t1162 - t1163 + t1164 + t1165 + t1166 + t1167 - t1168 - t1169 + t1170 + t1171 + t1172 - t1173 + t1174 + t602 - t1177 + f64x8::splat(4.0) * t631 + t1222 + t674;
            acc_v3rho3_2 = tv3rho32;
            let t1262 = f64x8::splat(3.0) * t1182;
            let t1263 = t680 + t685 + t689 - t692 + t695 - t698 + t1262 + t1122 - t720 + t727 - t735 - t738 + t740 + t748 - t776 - t781 - t789 - t791;
            let t1265 = t655 * t327;
            let t1271 = -f64x8::splat(6.0) * t353 - f64x8::splat(6.0) * t1134;
            let t1275 = ((t168).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1127 * t1265 + f64x8::splat(4.0) / f64x8::splat(3.0) * t632 * t659 + f64x8::splat(4.0) / f64x8::splat(3.0) * t171 * t1271));
            let t1276 = t664 * t331;
            let t1281 = -t1271;
            let t1285 = ((t175).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1142 * t1276 + f64x8::splat(4.0) / f64x8::splat(3.0) * t640 * t667 + f64x8::splat(4.0) / f64x8::splat(3.0) * t176 * t1281));
            let t1286 = t1275 + t1285;
            let t1288 = t163 * t1286 * t183;
            let t1289 = -t795 - t801 + t831 - t834 + t837 + t879 - t886 + t893 - t932 + t936 + t940 - t945 + t949 - t952 + f64x8::splat(3.0) * t1226 + t1288 - t957 - t1117;
            let tv3rho33 = t20 * (t1263 + t1289) + t1161 - t1162 - t1163 + t1164 + t1165 + t1166 + t1167 - t1168 - t1169 + t1170 + t1171 + t1172 - t1173 + t1174 - t1177 + f64x8::splat(6.0) * t631 + f64x8::splat(3.0) * t674;
            acc_v3rho3_3 = tv3rho33;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}
