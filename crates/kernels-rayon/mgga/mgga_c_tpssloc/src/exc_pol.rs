//! MGGA_C_TPSSLOC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_tpssloc.c`
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
pub fn mgga_c_tpssloc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t7 = (((f64x8::splat(0.0)).simd_lt(t5)).select(t5, -t5));
            let t8 = (-t7).simd_le(-f64x8::splat(0.999999999999));
            let t9 = t2 * t2;
            let t10 = t3 * t3;
            let t11 = f64x8::splat(1.0) / t10;
            let t14 = t9 * t9;
            let t15 = t10 * t10;
            let t16 = f64x8::splat(1.0) / t15;
            let t17 = t14 * t16;
            let t19 = t14 * t9;
            let t20 = t15 * t10;
            let t21 = f64x8::splat(1.0) / t20;
            let t24 = f64x8::splat(0.35) + f64x8::splat(0.87) * t9 * t11 + f64x8::splat(0.5) * t17 + f64x8::splat(2.26) * t19 * t21;
            let t25 = f64x8::splat(1.0) + t5;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = zeta_threshold - f64x8::splat(1.0);
            let t28 = f64x8::splat(1.0) - t5;
            let t29 = (t28).simd_le(zeta_threshold);
            let t31 = ((t26).select(t27, (t29).select(-t27, t5)));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) - t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = f64x8::splat(1.0) + t31;
            let t41 = t40 / f64x8::splat(2.0);
            let t42 = (simd::cbrt(t41));
            let t43 = t42 * t42;
            let t44 = t43 * t41;
            let t46 = v_rho1 * v_rho1;
            let t47 = (simd::cbrt(v_rho1));
            let t48 = t47 * t47;
            let t50 = f64x8::splat(1.0) / t48 / t46;
            let t51 = v_sigma2 * t50;
            let t52 = f64x8::splat(1.0) - t31;
            let t53 = t52 / f64x8::splat(2.0);
            let t54 = (simd::cbrt(t53));
            let t55 = t54 * t54;
            let t56 = t55 * t53;
            let t59 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t60 = (simd::cbrt(t3));
            let t61 = t60 * t60;
            let t63 = f64x8::splat(1.0) / t61 / t10;
            let t64 = t59 * t63;
            let t65 = t39 * t44 + t51 * t56 - t64;
            let t66 = t33 * t65;
            let t67 = f64x8::splat(M_CBRT3);
            let t68 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t69 = (simd::cbrt(t68));
            let t70 = t69 * t69;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t67 * t71;
            let t73 = (simd::cbrt(t40));
            let t74 = t73 * t40;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = (simd::cbrt(t52));
            let t77 = t76 * t52;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t75 + t78;
            let t80 = t72 * t79;
            let t83 = f64x8::splat(1.0) + t66 * t80 / f64x8::splat(24.0);
            let t84 = t83 * t83;
            let t85 = t84 * t84;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = ((t8).select(f64x8::splat(3.98), t24 * t86));
            let t89 = f64x8::splat(1.0) + t88;
            let t91 = f64x8::splat(1.0) / t36 / v_rho0;
            let t92 = v_tau0 * t91;
            let t93 = t25 / f64x8::splat(2.0);
            let t94 = (simd::cbrt(t93));
            let t95 = t94 * t94;
            let t96 = t95 * t93;
            let t99 = f64x8::splat(1.0) / t48 / v_rho1;
            let t100 = v_tau1 * t99;
            let t101 = t28 / f64x8::splat(2.0);
            let t102 = (simd::cbrt(t101));
            let t103 = t102 * t102;
            let t104 = t103 * t101;
            let t106 = t100 * t104 + t92 * t96;
            let t107 = f64x8::splat(1.0) / t106;
            let t109 = t64 * t107 / f64x8::splat(8.0);
            let t110 = (f64x8::splat(1.0)).simd_lt(t109);
            let t111 = ((t110).select(f64x8::splat(1.0), t109));
            let t112 = t111 * t111;
            let t113 = t89 * t112;
            let t115 = ((v_rho0).simd_le(dens_threshold)) | (t26);
            let t116 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t117 = (simd::cbrt(t116));
            let t118 = t67 * t117;
            let t119 = f64x8::splat(M_CBRT4);
            let t120 = t119 * t119;
            let t121 = f64x8::splat(1.0) / t60;
            let t122 = t120 * t121;
            let t123 = t118 * t122;
            let t125 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t123;
            let t126 = ((t123).sqrt());
            let t129 = ((t123) * (t123).sqrt());
            let t131 = t67 * t67;
            let t132 = t117 * t117;
            let t133 = t131 * t132;
            let t134 = f64x8::splat(1.0) / t61;
            let t135 = t119 * t134;
            let t136 = t133 * t135;
            let t138 = f64x8::splat(3.79785) * t126 + f64x8::splat(0.8969) * t123 + f64x8::splat(0.204775) * t129 + f64x8::splat(0.123235) * t136;
            let t141 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t138;
            let t142 = (simd::ln(t141));
            let t144 = f64x8::splat(0.0621814) * t125 * t142;
            let t145 = t32 * t32;
            let t146 = (t40).simd_le(zeta_threshold);
            let t147 = (simd::cbrt(zeta_threshold));
            let t148 = t147 * zeta_threshold;
            let t149 = ((t146).select(t148, t74));
            let t150 = (t52).simd_le(zeta_threshold);
            let t151 = ((t150).select(t148, t77));
            let t152 = t149 + t151 - f64x8::splat(2.0);
            let t153 = t145 * t152;
            let t154 = f64x8::splat(M_CBRT2);
            let t157 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t154 - f64x8::splat(2.0));
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t123;
            let t164 = f64x8::splat(7.05945) * t126 + f64x8::splat(1.549425) * t123 + f64x8::splat(0.420775) * t129 + f64x8::splat(0.1562925) * t136;
            let t167 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t164;
            let t168 = (simd::ln(t167));
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t123;
            let t177 = f64x8::splat(5.1785) * t126 + f64x8::splat(0.905775) * t123 + f64x8::splat(0.1100325) * t129 + f64x8::splat(0.1241775) * t136;
            let t180 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t177;
            let t181 = (simd::ln(t180));
            let t182 = t172 * t181;
            let t184 = -f64x8::splat(0.0310907) * t159 * t168 + t144 - f64x8::splat(0.0197516734986138) * t182;
            let t185 = t157 * t184;
            let t186 = t153 * t185;
            let t187 = t152 * t157;
            let t189 = f64x8::splat(0.0197516734986138) * t187 * t182;
            let t190 = (simd::ln(f64x8::splat(2.0)));
            let t191 = f64x8::splat(1.0) - t190;
            let t192 = f64x8::splat(1.0) / t68;
            let t193 = t191 * t192;
            let t194 = t147 * t147;
            let t195 = t73 * t73;
            let t196 = ((t146).select(t194, t195));
            let t197 = t76 * t76;
            let t198 = ((t150).select(t194, t197));
            let t200 = t196 / f64x8::splat(2.0) + t198 / f64x8::splat(2.0);
            let t201 = t200 * t200;
            let t202 = t201 * t200;
            let t204 = f64x8::splat(1.0) / t60 / t10;
            let t205 = t59 * t204;
            let t206 = f64x8::splat(1.0) / t201;
            let t207 = t154 * t206;
            let t209 = f64x8::splat(1.0) / t117;
            let t210 = t131 * t209;
            let t212 = (simd::exp(-t136 / f64x8::splat(4.0)));
            let t213 = f64x8::splat(1.0) - t212;
            let t214 = t119 * t213;
            let t215 = t210 * t214;
            let t218 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t205 * t207 * t215;
            let t219 = t205 * t154;
            let t220 = t206 * t131;
            let t221 = t209 * t119;
            let t222 = t220 * t221;
            let t225 = f64x8::splat(1.0) / t191;
            let t226 = t218 * t225;
            let t228 = (-t144 + t186 + t189) * t225;
            let t229 = f64x8::splat(1.0) / t202;
            let t230 = t68 * t229;
            let t232 = (simd::exp(-t228 * t230));
            let t233 = t232 - f64x8::splat(1.0);
            let t234 = f64x8::splat(1.0) / t233;
            let t235 = t68 * t234;
            let t236 = t59 * t59;
            let t237 = t235 * t236;
            let t238 = t226 * t237;
            let t240 = f64x8::splat(1.0) / t61 / t15;
            let t241 = t154 * t154;
            let t242 = t240 * t241;
            let t243 = t201 * t201;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = f64x8::splat(1.0) / t132;
            let t247 = t67 * t246;
            let t248 = t247 * t120;
            let t249 = t242 * t244 * t248;
            let t252 = t219 * t222 / f64x8::splat(96.0) + t238 * t249 / f64x8::splat(3072.0);
            let t253 = t218 * t252;
            let t254 = t225 * t68;
            let t255 = t235 * t252;
            let t257 = t226 * t255 + f64x8::splat(1.0);
            let t258 = f64x8::splat(1.0) / t257;
            let t259 = t254 * t258;
            let t261 = t253 * t259 + f64x8::splat(1.0);
            let t262 = (simd::ln(t261));
            let t265 = t193 * t202 * t262 - t144 + t186 + t189;
            let t268 = t118 * t120;
            let t269 = t121 * t154;
            let t270 = f64x8::splat(1.0) / t40;
            let t271 = (simd::cbrt(t270));
            let t273 = t268 * t269 * t271;
            let t275 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t273;
            let t276 = ((t273).sqrt());
            let t279 = ((t273) * (t273).sqrt());
            let t281 = t133 * t119;
            let t282 = t134 * t241;
            let t283 = t271 * t271;
            let t285 = t281 * t282 * t283;
            let t287 = f64x8::splat(3.79785) * t276 + f64x8::splat(0.8969) * t273 + f64x8::splat(0.204775) * t279 + f64x8::splat(0.123235) * t285;
            let t290 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t287;
            let t291 = (simd::ln(t290));
            let t293 = f64x8::splat(0.0621814) * t275 * t291;
            let t294 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t296 = ((t294).select(t148, f64x8::splat(2.0) * t154));
            let t297 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t298 = ((t297).select(t148, f64x8::splat(0.0)));
            let t300 = (t296 + t298 - f64x8::splat(2.0)) * t157;
            let t302 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t273;
            let t307 = f64x8::splat(7.05945) * t276 + f64x8::splat(1.549425) * t273 + f64x8::splat(0.420775) * t279 + f64x8::splat(0.1562925) * t285;
            let t310 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t307;
            let t311 = (simd::ln(t310));
            let t315 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t273;
            let t320 = f64x8::splat(5.1785) * t276 + f64x8::splat(0.905775) * t273 + f64x8::splat(0.1100325) * t279 + f64x8::splat(0.1241775) * t285;
            let t323 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t320;
            let t324 = (simd::ln(t323));
            let t325 = t315 * t324;
            let t328 = t300 * (-f64x8::splat(0.0310907) * t302 * t311 + t293 - f64x8::splat(0.0197516734986138) * t325);
            let t330 = f64x8::splat(0.0197516734986138) * t300 * t325;
            let t331 = ((t294).select(t194, t241));
            let t332 = ((t297).select(t194, f64x8::splat(0.0)));
            let t334 = t331 / f64x8::splat(2.0) + t332 / f64x8::splat(2.0);
            let t335 = t334 * t334;
            let t336 = t335 * t334;
            let t337 = f64x8::splat(1.0) / t335;
            let t338 = t337 * t131;
            let t339 = t39 * t338;
            let t340 = f64x8::splat(1.0) / t271;
            let t341 = t60 * t340;
            let t343 = (simd::exp(-t285 / f64x8::splat(4.0)));
            let t344 = f64x8::splat(1.0) - t343;
            let t345 = t341 * t344;
            let t346 = t221 * t345;
            let t349 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t339 * t346;
            let t350 = t221 * t341;
            let t353 = t349 * t225;
            let t354 = t353 * t68;
            let t357 = f64x8::splat(1.0) / t336;
            let t358 = t68 * t357;
            let t360 = (simd::exp(-(-t293 + t328 + t330) * t225 * t358));
            let t361 = t360 - f64x8::splat(1.0);
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = v_sigma0 * v_sigma0;
            let t364 = t362 * t363;
            let t365 = t34 * t34;
            let t366 = t365 * v_rho0;
            let t368 = f64x8::splat(1.0) / t35 / t366;
            let t369 = t364 * t368;
            let t370 = t354 * t369;
            let t371 = t335 * t335;
            let t372 = f64x8::splat(1.0) / t371;
            let t373 = t372 * t67;
            let t374 = t373 * t246;
            let t375 = t120 * t61;
            let t376 = f64x8::splat(1.0) / t283;
            let t378 = t374 * t375 * t376;
            let t381 = t339 * t350 / f64x8::splat(96.0) + t370 * t378 / f64x8::splat(3072.0);
            let t382 = t349 * t381;
            let t383 = t68 * t362;
            let t384 = t383 * t381;
            let t386 = t353 * t384 + f64x8::splat(1.0);
            let t387 = f64x8::splat(1.0) / t386;
            let t388 = t254 * t387;
            let t390 = t382 * t388 + f64x8::splat(1.0);
            let t391 = (simd::ln(t390));
            let t394 = t193 * t336 * t391 - t293 + t328 + t330;
            let t395 = (t265).simd_lt(t394);
            let t396 = ((t395).select(t394, t265));
            let t399 = ((t115).select(t265 * t25 / f64x8::splat(2.0), t396 * t40 / f64x8::splat(2.0)));
            let t401 = ((v_rho1).simd_le(dens_threshold)) | (t29);
            let t404 = f64x8::splat(1.0) / t52;
            let t405 = (simd::cbrt(t404));
            let t407 = t268 * t269 * t405;
            let t409 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t407;
            let t410 = ((t407).sqrt());
            let t413 = ((t407) * (t407).sqrt());
            let t415 = t405 * t405;
            let t417 = t281 * t282 * t415;
            let t419 = f64x8::splat(3.79785) * t410 + f64x8::splat(0.8969) * t407 + f64x8::splat(0.204775) * t413 + f64x8::splat(0.123235) * t417;
            let t422 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t419;
            let t423 = (simd::ln(t422));
            let t425 = f64x8::splat(0.0621814) * t409 * t423;
            let t427 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t407;
            let t432 = f64x8::splat(7.05945) * t410 + f64x8::splat(1.549425) * t407 + f64x8::splat(0.420775) * t413 + f64x8::splat(0.1562925) * t417;
            let t435 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t432;
            let t436 = (simd::ln(t435));
            let t440 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t407;
            let t445 = f64x8::splat(5.1785) * t410 + f64x8::splat(0.905775) * t407 + f64x8::splat(0.1100325) * t413 + f64x8::splat(0.1241775) * t417;
            let t448 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t445;
            let t449 = (simd::ln(t448));
            let t450 = t440 * t449;
            let t453 = t300 * (-f64x8::splat(0.0310907) * t427 * t436 + t425 - f64x8::splat(0.0197516734986138) * t450);
            let t455 = f64x8::splat(0.0197516734986138) * t300 * t450;
            let t456 = t51 * t338;
            let t457 = f64x8::splat(1.0) / t405;
            let t458 = t60 * t457;
            let t460 = (simd::exp(-t417 / f64x8::splat(4.0)));
            let t461 = f64x8::splat(1.0) - t460;
            let t462 = t458 * t461;
            let t463 = t221 * t462;
            let t466 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t456 * t463;
            let t467 = t221 * t458;
            let t470 = t466 * t225;
            let t471 = t470 * t68;
            let t475 = (simd::exp(-(-t425 + t453 + t455) * t225 * t358));
            let t476 = t475 - f64x8::splat(1.0);
            let t477 = f64x8::splat(1.0) / t476;
            let t478 = v_sigma2 * v_sigma2;
            let t479 = t477 * t478;
            let t480 = t46 * t46;
            let t481 = t480 * v_rho1;
            let t483 = f64x8::splat(1.0) / t47 / t481;
            let t484 = t479 * t483;
            let t485 = t471 * t484;
            let t486 = f64x8::splat(1.0) / t415;
            let t488 = t374 * t375 * t486;
            let t491 = t456 * t467 / f64x8::splat(96.0) + t485 * t488 / f64x8::splat(3072.0);
            let t492 = t466 * t491;
            let t493 = t68 * t477;
            let t494 = t493 * t491;
            let t496 = t470 * t494 + f64x8::splat(1.0);
            let t497 = f64x8::splat(1.0) / t496;
            let t498 = t254 * t497;
            let t500 = t492 * t498 + f64x8::splat(1.0);
            let t501 = (simd::ln(t500));
            let t504 = t193 * t336 * t501 - t425 + t453 + t455;
            let t505 = (t265).simd_lt(t504);
            let t506 = ((t505).select(t504, t265));
            let t509 = ((t401).select(t265 * t28 / f64x8::splat(2.0), t506 * t52 / f64x8::splat(2.0)));
            let t510 = t399 + t509;
            let t513 = t112 * t88 + f64x8::splat(1.0);
            let t514 = (simd::cbrt(t25));
            let t515 = t514 * t25;
            let t516 = ((t26).select(t148, t515));
            let t517 = (simd::cbrt(t28));
            let t518 = t517 * t28;
            let t519 = ((t29).select(t148, t518));
            let t520 = t516 + t519 - f64x8::splat(2.0);
            let t521 = t520 * t157;
            let t522 = t521 * t184;
            let t523 = t17 * t522;
            let t525 = f64x8::splat(0.0197516734986138) * t521 * t182;
            let t526 = t514 * t514;
            let t527 = ((t26).select(t194, t526));
            let t528 = t517 * t517;
            let t529 = ((t29).select(t194, t528));
            let t531 = t527 / f64x8::splat(2.0) + t529 / f64x8::splat(2.0);
            let t532 = t531 * t531;
            let t533 = t532 * t531;
            let t534 = f64x8::splat(1.0) / t532;
            let t535 = t154 * t534;
            let t539 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t205 * t535 * t215;
            let t540 = t534 * t131;
            let t541 = t540 * t221;
            let t544 = t539 * t225;
            let t546 = (-t144 + t523 + t525) * t225;
            let t547 = f64x8::splat(1.0) / t533;
            let t548 = t68 * t547;
            let t550 = (simd::exp(-t546 * t548));
            let t551 = t550 - f64x8::splat(1.0);
            let t552 = f64x8::splat(1.0) / t551;
            let t553 = t68 * t552;
            let t554 = t553 * t236;
            let t555 = t544 * t554;
            let t556 = t532 * t532;
            let t557 = f64x8::splat(1.0) / t556;
            let t559 = t242 * t557 * t248;
            let t562 = t219 * t541 / f64x8::splat(96.0) + t555 * t559 / f64x8::splat(3072.0);
            let t563 = t539 * t562;
            let t564 = t553 * t562;
            let t566 = t544 * t564 + f64x8::splat(1.0);
            let t567 = f64x8::splat(1.0) / t566;
            let t568 = t254 * t567;
            let t570 = t563 * t568 + f64x8::splat(1.0);
            let t571 = (simd::ln(t570));
            let t574 = t193 * t533 * t571 - t144 + t523 + t525;
            let t576 = -t113 * t510 + t513 * t574;
            let t577 = t112 * t111;
            let t580 = f64x8::splat(1.0) + f64x8::splat(4.5) * t576 * t577;
            let tzk0 = t576 * t580;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
