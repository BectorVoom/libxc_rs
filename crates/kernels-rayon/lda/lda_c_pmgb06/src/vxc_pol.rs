//! LDA_C_PMGB06 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pmgb06.c`
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
pub fn lda_c_pmgb06_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t5));
            let t10 = t9 * t9;
            let t11 = ((t6).select(t8, t10));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t16 = ((t13).select(t8, t15));
            let t18 = t11 / f64x8::splat(2.0) + t16 / f64x8::splat(2.0);
            let t19 = t18 * t18;
            let t20 = t19 * t18;
            let t21 = (simd::ln(f64x8::splat(2.0)));
            let t22 = t21 - f64x8::splat(1.0);
            let t23 = f64x8::splat(2.0) * t22;
            let t24 = t20 * t23;
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = f64x8::splat(M_CBRT3);
            let t28 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t27 * t29;
            let t31 = f64x8::splat(M_CBRT4);
            let t32 = t31 * t31;
            let t33 = (simd::cbrt(t2));
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t36 = t30 * t35;
            let t37 = ((t36).sqrt());
            let t38 = param_hyb_omega_0 * t37;
            let t39 = f64x8::splat(1.0) / t18;
            let t41 = f64x8::splat(2.923025) * t38 * t39;
            let t43 = (simd::cbrt(f64x8::splat(9.0)));
            let t44 = t43 * t43;
            let t52 = param_hyb_omega_0 * param_hyb_omega_0;
            let t53 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t31 * t44 * t29 / t22 / f64x8::splat(12.0)) * t52;
            let t54 = t53 * t27;
            let t55 = t29 * t32;
            let t56 = f64x8::splat(1.0) / t19;
            let t61 = t52 * param_hyb_omega_0;
            let t62 = t37 * t36;
            let t63 = t61 * t62;
            let t64 = f64x8::splat(1.0) / t20;
            let t67 = f64x8::splat(1.0) + t41 + t54 * t55 * t34 * t56 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t63 * t64;
            let t68 = t52 * t27;
            let t69 = t68 * t29;
            let t73 = f64x8::splat(1.0) + t41 + f64x8::splat(0.8621275) * t69 * t35 * t56;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = (simd::ln(t67 * t74));
            let t77 = t26 * t76;
            let t79 = t1 * t1;
            let t80 = t2 * t2;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t79 * t81;
            let t83 = f64x8::splat(1.0) - t82;
            let t84 = t3 * t83;
            let t93 = (f64x8::splat(2.0) / f64x8::splat(45.0) * t31 * t44 * t29 * (t25 + f64x8::splat(6.0) * t21 - f64x8::splat(3.0)) * t28 - f64x8::splat(0.7524)) * t27;
            let t97 = t27 * t27;
            let t98 = t29 * t29;
            let t99 = t97 * t98;
            let t100 = t33 * t33;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t31 * t101;
            let t103 = t99 * t102;
            let t106 = t29 * t28;
            let t107 = t27 * t106;
            let t109 = f64x8::splat(1.0) / t33 / t2;
            let t110 = t32 * t109;
            let t113 = f64x8::splat(1.0) - t93 * t55 * t34 / f64x8::splat(4.0) + f64x8::splat(0.0204825) * t103 - f64x8::splat(0.0030486129349252553) * t3 + f64x8::splat(0.0003485625) * t107 * t110;
            let t115 = (simd::exp(-f64x8::splat(0.1881) * t36));
            let t116 = t113 * t115;
            let t117 = f64x8::splat(M_SQRT2);
            let t118 = t116 * t117;
            let t122 = t97 * t98 * t26;
            let t123 = t122 * t31;
            let t125 = f64x8::splat(1.0) / t100 / t2;
            let t126 = zeta_threshold * zeta_threshold;
            let t127 = t5 * t5;
            let t128 = ((t6).select(t126, t127));
            let t129 = t128 * t44;
            let t130 = f64x8::splat(1.0) / t106;
            let t131 = t130 * t27;
            let t132 = t129 * t131;
            let t133 = f64x8::splat(1.0) / t5;
            let t134 = (simd::cbrt(t133));
            let t135 = t134 * t134;
            let t136 = f64x8::splat(1.0) / t135;
            let t137 = t100 * t136;
            let t138 = t30 * t32;
            let t139 = f64x8::splat(M_CBRT2);
            let t140 = t34 * t139;
            let t142 = t138 * t140 * t134;
            let t144 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t142;
            let t146 = t99 * t31;
            let t147 = t139 * t139;
            let t148 = t101 * t147;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t142 + f64x8::splat(0.01) * t146 * t148 * t135;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t144 * t153;
            let t155 = t137 * t154;
            let t157 = t132 * t155 / f64x8::splat(30.0);
            let t158 = t12 * t12;
            let t159 = ((t13).select(t126, t158));
            let t160 = t159 * t44;
            let t161 = t160 * t131;
            let t162 = f64x8::splat(1.0) / t12;
            let t163 = (simd::cbrt(t162));
            let t164 = t163 * t163;
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t100 * t165;
            let t168 = t138 * t140 * t163;
            let t170 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t168;
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t168 + f64x8::splat(0.01) * t146 * t148 * t164;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t170 * t176;
            let t178 = t166 * t177;
            let t180 = t161 * t178 / f64x8::splat(30.0);
            let t183 = -f64x8::splat(1.2375) * t36 + t103 / f64x8::splat(4.0);
            let t184 = t83 * t183;
            let t186 = (simd::exp(-f64x8::splat(0.0775) * t36));
            let t187 = t186 * f64x8::splat(M_PI);
            let t188 = t187 * t2;
            let t191 = t157 + t180 + f64x8::splat(4.0) / f64x8::splat(3.0) * t184 * t188;
            let t199 = t116 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0) + t82 / f64x8::splat(2.0);
            let t202 = t31 * t125;
            let t205 = -f64x8::splat(0.097) * t36 + f64x8::splat(0.169) * t103;
            let t206 = t83 * t205;
            let t208 = (simd::exp(-f64x8::splat(0.13675) * t36));
            let t209 = t206 * t208;
            let t211 = t27 / t98;
            let t213 = t211 * t32 * t100;
            let t216 = t8 * t126;
            let t217 = t10 * t127;
            let t218 = ((t6).select(t216, t217));
            let t219 = t15 * t158;
            let t220 = ((t13).select(t216, t219));
            let t223 = (t218 / f64x8::splat(2.0) + t220 / f64x8::splat(2.0)) * t44;
            let t224 = t131 * t100;
            let t227 = t157 + t180 + t209 * t213 / f64x8::splat(3.0) - t223 * t224 / f64x8::splat(15.0);
            let t232 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t36;
            let t235 = ((t36) * (t36).sqrt());
            let t238 = f64x8::splat(3.79785) * t37 + f64x8::splat(0.8969) * t36 + f64x8::splat(0.204775) * t235 + f64x8::splat(0.123235) * t103;
            let t241 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t238;
            let t242 = (simd::ln(t241));
            let t244 = f64x8::splat(0.0621814) * t232 * t242;
            let t245 = t79 * t79;
            let t246 = t80 * t80;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t245 * t247;
            let t249 = t7 * zeta_threshold;
            let t250 = t9 * t5;
            let t251 = ((t6).select(t249, t250));
            let t252 = t14 * t12;
            let t253 = ((t13).select(t249, t252));
            let t254 = t251 + t253 - f64x8::splat(2.0);
            let t257 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t139 - f64x8::splat(2.0));
            let t258 = t254 * t257;
            let t260 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t36;
            let t265 = f64x8::splat(7.05945) * t37 + f64x8::splat(1.549425) * t36 + f64x8::splat(0.420775) * t235 + f64x8::splat(0.1562925) * t103;
            let t268 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t265;
            let t269 = (simd::ln(t268));
            let t273 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t36;
            let t278 = f64x8::splat(5.1785) * t37 + f64x8::splat(0.905775) * t36 + f64x8::splat(0.1100325) * t235 + f64x8::splat(0.1241775) * t103;
            let t281 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t278;
            let t282 = (simd::ln(t281));
            let t283 = t273 * t282;
            let t285 = -f64x8::splat(0.0310907) * t260 * t269 + t244 - f64x8::splat(0.0197516734986138) * t283;
            let t286 = t258 * t285;
            let t290 = -t244 + t248 * t286 + f64x8::splat(0.0197516734986138) * t258 * t283;
            let t295 = t52 * t52;
            let t297 = t122 * t202;
            let t299 = t115 * t117;
            let t301 = t299 * t295 * param_hyb_omega_0;
            let t302 = t83 * t113 * t301;
            let t305 = t125 * t83;
            let t312 = t295 * t52;
            let t315 = f64x8::splat(1.0) / t100 / t80;
            let t317 = t295 * t295;
            let t321 = t24 * t77 + (-f64x8::splat(0.031505407223141116) * t84 * t118 - f64x8::splat(0.005388405304614574) * t123 * t125 * t191 * t117) * t61 + (-f64x8::splat(0.0837628205355044) * t84 * t199 - f64x8::splat(0.011938374665504766) * t122 * t202 * t227 + f64x8::splat(0.42708890021612717) * t107 * t110 * t290) * t295 - f64x8::splat(0.01197423401025461) * t297 * t302 + (-f64x8::splat(0.031835665774679375) * t123 * t305 * t199 + f64x8::splat(0.05332506774217938) * t81 * t290) * t312 + f64x8::splat(0.020267214298646783) * t123 * t315 * t290 * t317;
            let t325 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t99 * t102 * t52;
            let t326 = t325 * t325;
            let t327 = t326 * t326;
            let t328 = f64x8::splat(1.0) / t327;
            let tzk0 = t321 * t328;
            acc_zk = tzk0;
            let t329 = t19 * t23;
            let t330 = f64x8::splat(1.0) / t9;
            let t331 = t1 * t81;
            let t332 = t3 - t331;
            let t335 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t330 * t332));
            let t336 = f64x8::splat(1.0) / t14;
            let t337 = -t332;
            let t340 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t336 * t337));
            let t342 = t335 / f64x8::splat(2.0) + t340 / f64x8::splat(2.0);
            let t346 = t24 * t26;
            let t347 = f64x8::splat(1.0) / t37;
            let t348 = param_hyb_omega_0 * t347;
            let t349 = t348 * t39;
            let t350 = t30 * t110;
            let t352 = f64x8::splat(0.48717083333333333) * t349 * t350;
            let t355 = f64x8::splat(2.923025) * t38 * t56 * t342;
            let t359 = t54 * t55 * t109 * t56 / f64x8::splat(12.0);
            let t360 = t53 * t30;
            let t361 = t64 * t342;
            let t362 = t35 * t361;
            let t365 = t61 * t37;
            let t366 = t365 * t64;
            let t368 = f64x8::splat(0.24484) * t366 * t350;
            let t369 = t19 * t19;
            let t370 = f64x8::splat(1.0) / t369;
            let t371 = t370 * t342;
            let t374 = -t352 - t355 - t359 - t360 * t362 / f64x8::splat(2.0) - t368 - f64x8::splat(1.46904) * t63 * t371;
            let t376 = t73 * t73;
            let t377 = f64x8::splat(1.0) / t376;
            let t378 = t67 * t377;
            let t381 = f64x8::splat(0.28737583333333333) * t69 * t110 * t56;
            let t384 = -t352 - t355 - t381 - f64x8::splat(1.724255) * t69 * t362;
            let t386 = t374 * t74 - t378 * t384;
            let t387 = f64x8::splat(1.0) / t67;
            let t388 = t386 * t387;
            let t389 = t388 * t73;
            let t391 = t81 * t83;
            let t393 = f64x8::splat(0.031505407223141116) * t391 * t118;
            let t394 = t80 * t2;
            let t395 = f64x8::splat(1.0) / t394;
            let t396 = t79 * t395;
            let t398 = -f64x8::splat(2.0) * t331 + f64x8::splat(2.0) * t396;
            let t399 = t3 * t398;
            let t402 = t55 * t109;
            let t405 = t99 * t202;
            let t409 = f64x8::splat(1.0) / t33 / t80;
            let t410 = t32 * t409;
            let t413 = t93 * t402 / f64x8::splat(12.0) - f64x8::splat(0.013655) * t405 + f64x8::splat(0.0030486129349252553) * t81 - f64x8::splat(0.00046475) * t107 * t410;
            let t414 = t413 * t115;
            let t415 = t414 * t117;
            let t417 = f64x8::splat(0.031505407223141116) * t84 * t415;
            let t418 = t409 * t83;
            let t419 = t113 * t27;
            let t421 = t55 * t299;
            let t423 = f64x8::splat(0.001975389032890948) * t418 * t419 * t421;
            let t427 = f64x8::splat(0.008980675507690957) * t123 * t315 * t191 * t117;
            let t430 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) * t5 * t332));
            let t431 = t430 * t44;
            let t432 = t431 * t131;
            let t434 = t432 * t155 / f64x8::splat(30.0);
            let t435 = t34 * t136;
            let t436 = t435 * t154;
            let t438 = t132 * t436 / f64x8::splat(45.0);
            let t439 = t129 * t224;
            let t441 = f64x8::splat(1.0) / t135 / t133;
            let t442 = t441 * t144;
            let t443 = f64x8::splat(1.0) / t127;
            let t444 = t153 * t443;
            let t445 = t444 * t332;
            let t446 = t442 * t445;
            let t448 = t439 * t446 / f64x8::splat(45.0);
            let t449 = t109 * t139;
            let t451 = t138 * t449 * t134;
            let t452 = f64x8::splat(0.0018891666666666666) * t451;
            let t453 = t139 * t136;
            let t454 = t443 * t332;
            let t455 = t453 * t454;
            let t456 = t36 * t455;
            let t458 = t452 + f64x8::splat(0.0018891666666666666) * t456;
            let t459 = t458 * t153;
            let t460 = t137 * t459;
            let t462 = t132 * t460 / f64x8::splat(30.0);
            let t463 = t152 * t152;
            let t464 = f64x8::splat(1.0) / t463;
            let t465 = t144 * t464;
            let t466 = f64x8::splat(0.035991666666666665) * t451;
            let t468 = t125 * t147;
            let t471 = f64x8::splat(0.006666666666666667) * t146 * t468 * t135;
            let t472 = f64x8::splat(1.0) / t134;
            let t473 = t147 * t472;
            let t474 = t473 * t454;
            let t477 = -t466 - f64x8::splat(0.035991666666666665) * t456 - t471 - f64x8::splat(0.006666666666666667) * t103 * t474;
            let t478 = t465 * t477;
            let t479 = t137 * t478;
            let t481 = t132 * t479 / f64x8::splat(30.0);
            let t484 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) * t12 * t337));
            let t485 = t484 * t44;
            let t486 = t485 * t131;
            let t488 = t486 * t178 / f64x8::splat(30.0);
            let t489 = t34 * t165;
            let t490 = t489 * t177;
            let t492 = t161 * t490 / f64x8::splat(45.0);
            let t493 = t160 * t224;
            let t495 = f64x8::splat(1.0) / t164 / t162;
            let t496 = t495 * t170;
            let t497 = f64x8::splat(1.0) / t158;
            let t498 = t176 * t497;
            let t499 = t498 * t337;
            let t500 = t496 * t499;
            let t502 = t493 * t500 / f64x8::splat(45.0);
            let t504 = t138 * t449 * t163;
            let t505 = f64x8::splat(0.0018891666666666666) * t504;
            let t506 = t139 * t165;
            let t507 = t497 * t337;
            let t508 = t506 * t507;
            let t509 = t36 * t508;
            let t511 = t505 + f64x8::splat(0.0018891666666666666) * t509;
            let t512 = t511 * t176;
            let t513 = t166 * t512;
            let t515 = t161 * t513 / f64x8::splat(30.0);
            let t516 = t175 * t175;
            let t517 = f64x8::splat(1.0) / t516;
            let t518 = t170 * t517;
            let t519 = f64x8::splat(0.035991666666666665) * t504;
            let t523 = f64x8::splat(0.006666666666666667) * t146 * t468 * t164;
            let t524 = f64x8::splat(1.0) / t163;
            let t525 = t147 * t524;
            let t526 = t525 * t507;
            let t529 = -t519 - f64x8::splat(0.035991666666666665) * t509 - t523 - f64x8::splat(0.006666666666666667) * t103 * t526;
            let t530 = t518 * t529;
            let t531 = t166 * t530;
            let t533 = t161 * t531 / f64x8::splat(30.0);
            let t534 = t398 * t183;
            let t539 = f64x8::splat(0.4125) * t350 - t405 / f64x8::splat(6.0);
            let t540 = t83 * t539;
            let t542 = f64x8::splat(4.0) / f64x8::splat(3.0) * t540 * t188;
            let t543 = t184 * t27;
            let t545 = t55 * t34 * t186;
            let t547 = f64x8::splat(0.10821041362364843) * t543 * t545;
            let t549 = f64x8::splat(4.0) / f64x8::splat(3.0) * t184 * t187;
            let t550 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + f64x8::splat(4.0) / f64x8::splat(3.0) * t534 * t188 + t542 + t547 + t549;
            let t558 = f64x8::splat(0.0837628205355044) * t391 * t199;
            let t561 = t414 / f64x8::splat(2.0);
            let t562 = t419 * t29;
            let t563 = t110 * t115;
            let t565 = f64x8::splat(0.03135) * t562 * t563;
            let t566 = t561 + t565 + t331 - t396;
            let t569 = t31 * t315;
            let t572 = f64x8::splat(0.019897291109174608) * t122 * t569 * t227;
            let t573 = t398 * t205;
            let t574 = t573 * t208;
            let t579 = f64x8::splat(0.03233333333333333) * t350 - f64x8::splat(0.11266666666666666) * t405;
            let t580 = t83 * t579;
            let t581 = t580 * t208;
            let t583 = t581 * t213 / f64x8::splat(3.0);
            let t584 = t206 * t97;
            let t586 = f64x8::splat(1.0) / t29 * t31;
            let t588 = t586 * t101 * t208;
            let t590 = f64x8::splat(0.06077777777777778) * t584 * t588;
            let t591 = t211 * t35;
            let t593 = f64x8::splat(2.0) / f64x8::splat(9.0) * t209 * t591;
            let t594 = t10 * t5;
            let t597 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t594 * t332));
            let t598 = t15 * t12;
            let t601 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t598 * t337));
            let t604 = (t597 / f64x8::splat(2.0) + t601 / f64x8::splat(2.0)) * t44;
            let t607 = t131 * t34;
            let t609 = f64x8::splat(2.0) / f64x8::splat(45.0) * t223 * t607;
            let t610 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + t574 * t213 / f64x8::splat(3.0) + t583 + t590 + t593 - t604 * t224 / f64x8::splat(15.0) - t609;
            let t616 = f64x8::splat(0.5694518669548363) * t107 * t410 * t290;
            let t619 = f64x8::splat(0.0011073470983333333) * t30 * t110 * t242;
            let t620 = t238 * t238;
            let t621 = f64x8::splat(1.0) / t620;
            let t622 = t232 * t621;
            let t623 = t347 * t27;
            let t624 = t623 * t402;
            let t627 = ((t36).sqrt());
            let t628 = t627 * t27;
            let t629 = t628 * t402;
            let t632 = -f64x8::splat(0.632975) * t624 - f64x8::splat(0.29896666666666666) * t350 - f64x8::splat(0.1023875) * t629 - f64x8::splat(0.08215666666666667) * t405;
            let t633 = f64x8::splat(1.0) / t241;
            let t634 = t632 * t633;
            let t636 = f64x8::splat(1.0) * t622 * t634;
            let t637 = t79 * t1;
            let t638 = t637 * t247;
            let t640 = f64x8::splat(4.0) * t638 * t286;
            let t641 = t246 * t2;
            let t642 = f64x8::splat(1.0) / t641;
            let t643 = t245 * t642;
            let t645 = f64x8::splat(4.0) * t643 * t286;
            let t648 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t332));
            let t651 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t337));
            let t653 = (t648 + t651) * t257;
            let t654 = t653 * t285;
            let t659 = t265 * t265;
            let t660 = f64x8::splat(1.0) / t659;
            let t661 = t260 * t660;
            let t666 = -f64x8::splat(1.176575) * t624 - f64x8::splat(0.516475) * t350 - f64x8::splat(0.2103875) * t629 - f64x8::splat(0.104195) * t405;
            let t667 = f64x8::splat(1.0) / t268;
            let t668 = t666 * t667;
            let t674 = t278 * t278;
            let t675 = f64x8::splat(1.0) / t674;
            let t676 = t273 * t675;
            let t681 = -f64x8::splat(0.8630833333333333) * t624 - f64x8::splat(0.301925) * t350 - f64x8::splat(0.05501625) * t629 - f64x8::splat(0.082785) * t405;
            let t682 = f64x8::splat(1.0) / t281;
            let t683 = t681 * t682;
            let t686 = f64x8::splat(0.0005323764196666666) * t30 * t110 * t269 + f64x8::splat(1.0) * t661 * t668 - t619 - t636 + f64x8::splat(0.00018311447306006544) * t30 * t110 * t282 + f64x8::splat(0.5848223622634646) * t676 * t683;
            let t687 = t258 * t686;
            let t688 = t248 * t687;
            let t691 = t258 * t27;
            let t693 = t55 * t109 * t282;
            let t695 = f64x8::splat(0.00018311447306006544) * t691 * t693;
            let t696 = t258 * t273;
            let t698 = t675 * t681 * t682;
            let t700 = f64x8::splat(0.5848223622634646) * t696 * t698;
            let t701 = t619 + t636 + t640 - t645 + t248 * t654 + t688 + f64x8::splat(0.0197516734986138) * t653 * t283 - t695 - t700;
            let t707 = t122 * t569;
            let t709 = f64x8::splat(0.019957056683757683) * t707 * t302;
            let t711 = t398 * t113 * t301;
            let t715 = t83 * t413 * t301;
            let t717 = f64x8::splat(0.01197423401025461) * t297 * t715;
            let t718 = t395 * t83;
            let t721 = f64x8::splat(0.0002905674151788692) * t718 * t113 * t301;
            let t722 = t315 * t83;
            let t725 = f64x8::splat(0.053059442957798957) * t123 * t722 * t199;
            let t726 = t125 * t398;
            let t734 = f64x8::splat(0.10665013548435875) * t395 * t290;
            let t740 = f64x8::splat(1.0) / t100 / t394;
            let t744 = f64x8::splat(0.054045904796391424) * t123 * t740 * t290 * t317;
            let t749 = f64x8::splat(3.0) * t329 * t77 * t342 + t346 * t389 + (t393 - f64x8::splat(0.031505407223141116) * t399 * t118 - t417 - t423 + t427 - f64x8::splat(0.005388405304614574) * t123 * t125 * t550 * t117) * t61 + (t558 - f64x8::splat(0.0837628205355044) * t399 * t199 - f64x8::splat(0.0837628205355044) * t84 * t566 + t572 - f64x8::splat(0.011938374665504766) * t122 * t202 * t610 - t616 + f64x8::splat(0.42708890021612717) * t107 * t110 * t701) * t295 + t709 - f64x8::splat(0.01197423401025461) * t297 * t711 - t717 - t721 + (t725 - f64x8::splat(0.031835665774679375) * t123 * t726 * t199 - f64x8::splat(0.031835665774679375) * t123 * t305 * t566 - t734 + f64x8::splat(0.05332506774217938) * t81 * t701) * t312 - t744 + f64x8::splat(0.020267214298646783) * t123 * t315 * t701 * t317;
            let t754 = f64x8::splat(1.0) / t327 / t325;
            let t757 = t99 * t31 * t52;
            let t759 = f64x8::splat(0.41076328840066667) * t101 * t321 * t754 * t757;
            let tvrho0 = t2 * t749 * t328 + t759 + tzk0;
            acc_vrho_0 = tvrho0;
            let t760 = -t3 - t331;
            let t761 = t330 * t760;
            let t763 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t761));
            let t764 = -t760;
            let t765 = t336 * t764;
            let t767 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t765));
            let t769 = t763 / f64x8::splat(2.0) + t767 / f64x8::splat(2.0);
            let t770 = t77 * t769;
            let t773 = t56 * t769;
            let t775 = f64x8::splat(2.923025) * t38 * t773;
            let t776 = t64 * t769;
            let t777 = t35 * t776;
            let t780 = t370 * t769;
            let t783 = -t352 - t775 - t359 - t360 * t777 / f64x8::splat(2.0) - t368 - f64x8::splat(1.46904) * t63 * t780;
            let t787 = -t352 - t775 - t381 - f64x8::splat(1.724255) * t69 * t777;
            let t789 = -t378 * t787 + t783 * t74;
            let t790 = t789 * t387;
            let t791 = t790 * t73;
            let t794 = f64x8::splat(2.0) * t331 + f64x8::splat(2.0) * t396;
            let t795 = t3 * t794;
            let t800 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) * t5 * t760));
            let t801 = t800 * t44;
            let t802 = t801 * t131;
            let t804 = t802 * t155 / f64x8::splat(30.0);
            let t805 = t444 * t760;
            let t806 = t442 * t805;
            let t808 = t439 * t806 / f64x8::splat(45.0);
            let t809 = t443 * t760;
            let t810 = t453 * t809;
            let t811 = t36 * t810;
            let t813 = t452 + f64x8::splat(0.0018891666666666666) * t811;
            let t814 = t813 * t153;
            let t815 = t137 * t814;
            let t817 = t132 * t815 / f64x8::splat(30.0);
            let t819 = t473 * t809;
            let t822 = -t466 - f64x8::splat(0.035991666666666665) * t811 - t471 - f64x8::splat(0.006666666666666667) * t103 * t819;
            let t823 = t465 * t822;
            let t824 = t137 * t823;
            let t826 = t132 * t824 / f64x8::splat(30.0);
            let t829 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) * t12 * t764));
            let t830 = t829 * t44;
            let t831 = t830 * t131;
            let t833 = t831 * t178 / f64x8::splat(30.0);
            let t834 = t498 * t764;
            let t835 = t496 * t834;
            let t837 = t493 * t835 / f64x8::splat(45.0);
            let t838 = t497 * t764;
            let t839 = t506 * t838;
            let t840 = t36 * t839;
            let t842 = t505 + f64x8::splat(0.0018891666666666666) * t840;
            let t843 = t842 * t176;
            let t844 = t166 * t843;
            let t846 = t161 * t844 / f64x8::splat(30.0);
            let t848 = t525 * t838;
            let t851 = -t519 - f64x8::splat(0.035991666666666665) * t840 - t523 - f64x8::splat(0.006666666666666667) * t103 * t848;
            let t852 = t518 * t851;
            let t853 = t166 * t852;
            let t855 = t161 * t853 / f64x8::splat(30.0);
            let t856 = t794 * t183;
            let t859 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + f64x8::splat(4.0) / f64x8::splat(3.0) * t856 * t188 + t542 + t547 + t549;
            let t868 = t561 + t565 - t331 - t396;
            let t871 = t794 * t205;
            let t872 = t871 * t208;
            let t877 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t594 * t760));
            let t880 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t598 * t764));
            let t883 = (t877 / f64x8::splat(2.0) + t880 / f64x8::splat(2.0)) * t44;
            let t886 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + t872 * t213 / f64x8::splat(3.0) + t583 + t590 + t593 - t883 * t224 / f64x8::splat(15.0) - t609;
            let t892 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t760));
            let t895 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t764));
            let t897 = (t892 + t895) * t257;
            let t898 = t897 * t285;
            let t902 = t619 + t636 - t640 - t645 + t248 * t898 + t688 + f64x8::splat(0.0197516734986138) * t897 * t283 - t695 - t700;
            let t909 = t794 * t113 * t301;
            let t912 = t125 * t794;
            let t927 = f64x8::splat(3.0) * t329 * t770 + t346 * t791 + (t393 - f64x8::splat(0.031505407223141116) * t795 * t118 - t417 - t423 + t427 - f64x8::splat(0.005388405304614574) * t123 * t125 * t859 * t117) * t61 + (t558 - f64x8::splat(0.0837628205355044) * t795 * t199 - f64x8::splat(0.0837628205355044) * t84 * t868 + t572 - f64x8::splat(0.011938374665504766) * t122 * t202 * t886 - t616 + f64x8::splat(0.42708890021612717) * t107 * t110 * t902) * t295 + t709 - f64x8::splat(0.01197423401025461) * t297 * t909 - t717 - t721 + (t725 - f64x8::splat(0.031835665774679375) * t123 * t912 * t199 - f64x8::splat(0.031835665774679375) * t123 * t305 * t868 - t734 + f64x8::splat(0.05332506774217938) * t81 * t902) * t312 - t744 + f64x8::splat(0.020267214298646783) * t123 * t315 * t902 * t317;
            let tvrho1 = t2 * t927 * t328 + t759 + tzk0;
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
