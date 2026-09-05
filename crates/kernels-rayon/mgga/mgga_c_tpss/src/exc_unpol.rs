//! MGGA_C_TPSS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_tpss.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_tpss_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_beta: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_C0_c_0 = f64x8::splat(param_C0_c_0);
    let param_C0_c_1 = f64x8::splat(param_C0_c_1);
    let param_C0_c_2 = f64x8::splat(param_C0_c_2);
    let param_C0_c_3 = f64x8::splat(param_C0_c_3);
    let param_beta = f64x8::splat(param_beta);
    let param_d = f64x8::splat(param_d);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (((f64x8::splat(0.0)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t4 = (-t3).simd_le(-f64x8::splat(0.999999999999));
            let t5 = param_C0_c_0;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t13 = ((t10).select(t11, (t10).select(-t11, f64x8::splat(0.0))));
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) - t14;
            let t16 = f64x8::splat(M_CBRT2);
            let t17 = t16 * t16;
            let t18 = v_sigma * t17;
            let t19 = v_rho * v_rho;
            let t20 = (simd::cbrt(v_rho));
            let t21 = t20 * t20;
            let t23 = f64x8::splat(1.0) / t21 / t19;
            let t24 = f64x8::splat(1.0) + t13;
            let t25 = t24 / f64x8::splat(2.0);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t26;
            let t28 = t27 * t25;
            let t31 = f64x8::splat(1.0) - t13;
            let t32 = t31 / f64x8::splat(2.0);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = t34 * t32;
            let t41 = f64x8::splat(M_CBRT3);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t41 * t45;
            let t47 = (simd::cbrt(t24));
            let t48 = t47 * t24;
            let t50 = (simd::cbrt(t31));
            let t51 = t50 * t31;
            let t53 = f64x8::splat(1.0) / t48 + f64x8::splat(1.0) / t51;
            let t57 = f64x8::splat(1.0) + t15 * (t18 * t23 * t28 + t18 * t23 * t35 - v_sigma * t23) * t46 * t53 / f64x8::splat(24.0);
            let t58 = t57 * t57;
            let t59 = t58 * t58;
            let t62 = ((t4).select(t5 + param_C0_c_1 + param_C0_c_2 + param_C0_c_3, t5 / t59));
            let t63 = f64x8::splat(1.0) + t62;
            let t64 = f64x8::splat(1.0) / v_rho;
            let t65 = v_sigma * t64;
            let t66 = f64x8::splat(1.0) / v_tau;
            let t68 = t65 * t66 / f64x8::splat(8.0);
            let t69 = (f64x8::splat(1.0)).simd_lt(t68);
            let t70 = ((t69).select(f64x8::splat(1.0), t68));
            let t71 = t70 * t70;
            let t72 = t63 * t71;
            let t75 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t10);
            let t76 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t77 = (simd::cbrt(t76));
            let t78 = t41 * t77;
            let t79 = f64x8::splat(M_CBRT4);
            let t80 = t79 * t79;
            let t81 = f64x8::splat(1.0) / t20;
            let t83 = t78 * t80 * t81;
            let t85 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t83;
            let t86 = ((t83).sqrt());
            let t89 = ((t83) * (t83).sqrt());
            let t91 = t41 * t41;
            let t92 = t77 * t77;
            let t93 = t91 * t92;
            let t94 = f64x8::splat(1.0) / t21;
            let t96 = t93 * t79 * t94;
            let t98 = f64x8::splat(3.79785) * t86 + f64x8::splat(0.8969) * t83 + f64x8::splat(0.204775) * t89 + f64x8::splat(0.123235) * t96;
            let t101 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t98;
            let t102 = (simd::ln(t101));
            let t103 = t85 * t102;
            let t105 = t14 * t14;
            let t106 = (t24).simd_le(zeta_threshold);
            let t107 = (simd::cbrt(zeta_threshold));
            let t108 = t107 * zeta_threshold;
            let t109 = ((t106).select(t108, t48));
            let t110 = (t31).simd_le(zeta_threshold);
            let t111 = ((t110).select(t108, t51));
            let t112 = t109 + t111 - f64x8::splat(2.0);
            let t113 = t105 * t112;
            let t116 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t16 - f64x8::splat(2.0));
            let t118 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t83;
            let t123 = f64x8::splat(7.05945) * t86 + f64x8::splat(1.549425) * t83 + f64x8::splat(0.420775) * t89 + f64x8::splat(0.1562925) * t96;
            let t126 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t123;
            let t127 = (simd::ln(t126));
            let t130 = f64x8::splat(0.0621814) * t103;
            let t132 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t83;
            let t137 = f64x8::splat(5.1785) * t86 + f64x8::splat(0.905775) * t83 + f64x8::splat(0.1100325) * t89 + f64x8::splat(0.1241775) * t96;
            let t140 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t137;
            let t141 = (simd::ln(t140));
            let t142 = t132 * t141;
            let t146 = t113 * t116 * (-f64x8::splat(0.0310907) * t118 * t127 + t130 - f64x8::splat(0.0197516734986138) * t142);
            let t148 = t112 * t116;
            let t149 = t148 * t142;
            let t151 = (simd::ln(f64x8::splat(2.0)));
            let t152 = f64x8::splat(1.0) - t151;
            let t153 = f64x8::splat(1.0) / t42;
            let t154 = t152 * t153;
            let t155 = t107 * t107;
            let t156 = t47 * t47;
            let t157 = ((t106).select(t155, t156));
            let t158 = t50 * t50;
            let t159 = ((t110).select(t155, t158));
            let t161 = t157 / f64x8::splat(2.0) + t159 / f64x8::splat(2.0);
            let t162 = t161 * t161;
            let t163 = t162 * t161;
            let t165 = f64x8::splat(1.0) / t20 / t19;
            let t166 = v_sigma * t165;
            let t167 = t166 * t16;
            let t168 = f64x8::splat(1.0) / t162;
            let t170 = f64x8::splat(1.0) / t77;
            let t171 = t170 * t79;
            let t172 = t168 * t91 * t171;
            let t175 = f64x8::splat(1.0) / t152;
            let t176 = param_beta * t175;
            let t177 = f64x8::splat(0.0197516734986138) * t149;
            let t180 = f64x8::splat(1.0) / t163;
            let t183 = (simd::exp(-(-t130 + t146 + t177) * t175 * t42 * t180));
            let t184 = t183 - f64x8::splat(1.0);
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t42 * t185;
            let t187 = v_sigma * v_sigma;
            let t189 = t176 * t186 * t187;
            let t190 = t19 * t19;
            let t192 = f64x8::splat(1.0) / t21 / t190;
            let t193 = t192 * t17;
            let t194 = t162 * t162;
            let t195 = f64x8::splat(1.0) / t194;
            let t196 = t193 * t195;
            let t197 = f64x8::splat(1.0) / t92;
            let t198 = t41 * t197;
            let t199 = t198 * t80;
            let t200 = t196 * t199;
            let t203 = t167 * t172 / f64x8::splat(96.0) + t189 * t200 / f64x8::splat(3072.0);
            let t204 = param_beta * t203;
            let t205 = t175 * t42;
            let t208 = t176 * t186 * t203 + f64x8::splat(1.0);
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t205 * t209;
            let t212 = t204 * t210 + f64x8::splat(1.0);
            let t213 = (simd::ln(t212));
            let t215 = t154 * t163 * t213;
            let t217 = -f64x8::splat(0.0310907) * t103 + t146 / f64x8::splat(2.0) + f64x8::splat(0.0098758367493069) * t149 + t215 / f64x8::splat(2.0);
            let t218 = -t130 + t146 + t177 + t215;
            let t219 = t78 * t80;
            let t220 = t81 * t16;
            let t221 = f64x8::splat(1.0) / t24;
            let t222 = (simd::cbrt(t221));
            let t224 = t219 * t220 * t222;
            let t226 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t224;
            let t227 = ((t224).sqrt());
            let t230 = ((t224) * (t224).sqrt());
            let t232 = t93 * t79;
            let t233 = t94 * t17;
            let t234 = t222 * t222;
            let t236 = t232 * t233 * t234;
            let t238 = f64x8::splat(3.79785) * t227 + f64x8::splat(0.8969) * t224 + f64x8::splat(0.204775) * t230 + f64x8::splat(0.123235) * t236;
            let t241 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t238;
            let t242 = (simd::ln(t241));
            let t244 = f64x8::splat(0.0621814) * t226 * t242;
            let t245 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t247 = ((t245).select(t108, f64x8::splat(2.0) * t16));
            let t248 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t249 = ((t248).select(t108, f64x8::splat(0.0)));
            let t251 = (t247 + t249 - f64x8::splat(2.0)) * t116;
            let t253 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t224;
            let t258 = f64x8::splat(7.05945) * t227 + f64x8::splat(1.549425) * t224 + f64x8::splat(0.420775) * t230 + f64x8::splat(0.1562925) * t236;
            let t261 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t258;
            let t262 = (simd::ln(t261));
            let t266 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t224;
            let t271 = f64x8::splat(5.1785) * t227 + f64x8::splat(0.905775) * t224 + f64x8::splat(0.1100325) * t230 + f64x8::splat(0.1241775) * t236;
            let t274 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t271;
            let t275 = (simd::ln(t274));
            let t276 = t266 * t275;
            let t279 = t251 * (-f64x8::splat(0.0310907) * t253 * t262 + t244 - f64x8::splat(0.0197516734986138) * t276);
            let t281 = f64x8::splat(0.0197516734986138) * t251 * t276;
            let t282 = ((t245).select(t155, t17));
            let t283 = ((t248).select(t155, f64x8::splat(0.0)));
            let t285 = t282 / f64x8::splat(2.0) + t283 / f64x8::splat(2.0);
            let t286 = t285 * t285;
            let t287 = t286 * t285;
            let t288 = f64x8::splat(1.0) / t286;
            let t289 = t288 * t91;
            let t290 = t166 * t289;
            let t293 = t171 * t17 / t222;
            let t296 = t176 * t42;
            let t299 = f64x8::splat(1.0) / t287;
            let t300 = t42 * t299;
            let t302 = (simd::exp(-(-t244 + t279 + t281) * t175 * t300));
            let t303 = t302 - f64x8::splat(1.0);
            let t304 = f64x8::splat(1.0) / t303;
            let t305 = t304 * t187;
            let t308 = t286 * t286;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t309 * t41;
            let t311 = t310 * t197;
            let t312 = t80 * t16;
            let t313 = f64x8::splat(1.0) / t234;
            let t315 = t311 * t312 * t313;
            let t318 = t290 * t293 / f64x8::splat(96.0) + t296 * t305 * t192 * t315 / f64x8::splat(1536.0);
            let t319 = param_beta * t318;
            let t320 = t42 * t304;
            let t323 = t176 * t320 * t318 + f64x8::splat(1.0);
            let t324 = f64x8::splat(1.0) / t323;
            let t325 = t205 * t324;
            let t327 = t319 * t325 + f64x8::splat(1.0);
            let t328 = (simd::ln(t327));
            let t331 = t154 * t287 * t328 - t244 + t279 + t281;
            let t332 = (t218).simd_lt(t331);
            let t333 = ((t332).select(t331, t218));
            let t336 = ((t75).select(t217, t333 * t24 / f64x8::splat(2.0)));
            let t337 = f64x8::splat(1.0) / t31;
            let t338 = (simd::cbrt(t337));
            let t340 = t219 * t220 * t338;
            let t342 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t340;
            let t343 = ((t340).sqrt());
            let t346 = ((t340) * (t340).sqrt());
            let t348 = t338 * t338;
            let t350 = t232 * t233 * t348;
            let t352 = f64x8::splat(3.79785) * t343 + f64x8::splat(0.8969) * t340 + f64x8::splat(0.204775) * t346 + f64x8::splat(0.123235) * t350;
            let t355 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t352;
            let t356 = (simd::ln(t355));
            let t358 = f64x8::splat(0.0621814) * t342 * t356;
            let t360 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t340;
            let t365 = f64x8::splat(7.05945) * t343 + f64x8::splat(1.549425) * t340 + f64x8::splat(0.420775) * t346 + f64x8::splat(0.1562925) * t350;
            let t368 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t365;
            let t369 = (simd::ln(t368));
            let t373 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t340;
            let t378 = f64x8::splat(5.1785) * t343 + f64x8::splat(0.905775) * t340 + f64x8::splat(0.1100325) * t346 + f64x8::splat(0.1241775) * t350;
            let t381 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t378;
            let t382 = (simd::ln(t381));
            let t383 = t373 * t382;
            let t386 = t251 * (-f64x8::splat(0.0310907) * t360 * t369 + t358 - f64x8::splat(0.0197516734986138) * t383);
            let t388 = f64x8::splat(0.0197516734986138) * t251 * t383;
            let t391 = t171 * t17 / t338;
            let t397 = (simd::exp(-(-t358 + t386 + t388) * t175 * t300));
            let t398 = t397 - f64x8::splat(1.0);
            let t399 = f64x8::splat(1.0) / t398;
            let t400 = t399 * t187;
            let t403 = f64x8::splat(1.0) / t348;
            let t405 = t311 * t312 * t403;
            let t408 = t290 * t391 / f64x8::splat(96.0) + t296 * t400 * t192 * t405 / f64x8::splat(1536.0);
            let t409 = param_beta * t408;
            let t410 = t42 * t399;
            let t413 = t176 * t410 * t408 + f64x8::splat(1.0);
            let t414 = f64x8::splat(1.0) / t413;
            let t415 = t205 * t414;
            let t417 = t409 * t415 + f64x8::splat(1.0);
            let t418 = (simd::ln(t417));
            let t421 = t154 * t287 * t418 - t358 + t386 + t388;
            let t422 = (t218).simd_lt(t421);
            let t423 = ((t422).select(t421, t218));
            let t426 = ((t75).select(t217, t423 * t31 / f64x8::splat(2.0)));
            let t427 = t336 + t426;
            let t430 = t62 * t71 + f64x8::splat(1.0);
            let t431 = ((t10).select(t108, f64x8::splat(1.0)));
            let t434 = (f64x8::splat(2.0) * t431 - f64x8::splat(2.0)) * t116;
            let t436 = f64x8::splat(0.0197516734986138) * t434 * t142;
            let t437 = ((t10).select(t155, f64x8::splat(1.0)));
            let t438 = t437 * t437;
            let t439 = t438 * t437;
            let t440 = f64x8::splat(1.0) / t438;
            let t442 = t440 * t91 * t171;
            let t447 = f64x8::splat(1.0) / t439;
            let t450 = (simd::exp(-(-t130 + t436) * t175 * t42 * t447));
            let t451 = t450 - f64x8::splat(1.0);
            let t452 = f64x8::splat(1.0) / t451;
            let t453 = t42 * t452;
            let t455 = t176 * t453 * t187;
            let t456 = t438 * t438;
            let t457 = f64x8::splat(1.0) / t456;
            let t458 = t193 * t457;
            let t459 = t458 * t199;
            let t462 = t167 * t442 / f64x8::splat(96.0) + t455 * t459 / f64x8::splat(3072.0);
            let t463 = param_beta * t462;
            let t466 = t176 * t453 * t462 + f64x8::splat(1.0);
            let t467 = f64x8::splat(1.0) / t466;
            let t468 = t205 * t467;
            let t470 = t463 * t468 + f64x8::splat(1.0);
            let t471 = (simd::ln(t470));
            let t474 = t154 * t439 * t471 - t130 + t436;
            let t476 = -t72 * t427 + t430 * t474;
            let t477 = param_d * t476;
            let t478 = t71 * t70;
            let t480 = t477 * t478 + f64x8::splat(1.0);
            let tzk0 = t476 * t480;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
