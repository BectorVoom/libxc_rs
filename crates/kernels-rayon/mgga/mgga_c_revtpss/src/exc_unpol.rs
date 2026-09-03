//! MGGA_C_REVTPSS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revtpss.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_revtpss_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_C0_c_0 = f64x8::splat(param_C0_c_0);
    let param_C0_c_1 = f64x8::splat(param_C0_c_1);
    let param_C0_c_2 = f64x8::splat(param_C0_c_2);
    let param_C0_c_3 = f64x8::splat(param_C0_c_3);
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
            let t165 = f64x8::splat(1.0) + f64x8::splat(0.025) * t83;
            let t167 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t83;
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t165 * t168;
            let t171 = f64x8::splat(1.0) / t20 / t19;
            let t172 = v_sigma * t171;
            let t173 = t172 * t16;
            let t174 = f64x8::splat(1.0) / t162;
            let t176 = f64x8::splat(1.0) / t77;
            let t177 = t176 * t79;
            let t178 = t174 * t91 * t177;
            let t181 = f64x8::splat(1.0) / t152;
            let t182 = f64x8::splat(0.0197516734986138) * t149;
            let t185 = f64x8::splat(1.0) / t163;
            let t186 = t42 * t185;
            let t188 = (simd::exp(-(-t130 + t146 + t182) * t181 * t186));
            let t189 = t188 - f64x8::splat(1.0);
            let t190 = f64x8::splat(1.0) / t189;
            let t191 = t181 * t190;
            let t192 = v_sigma * v_sigma;
            let t193 = t191 * t192;
            let t194 = t169 * t193;
            let t195 = t19 * t19;
            let t197 = f64x8::splat(1.0) / t21 / t195;
            let t198 = t197 * t17;
            let t199 = t162 * t162;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t198 * t200;
            let t202 = f64x8::splat(1.0) / t92;
            let t203 = t41 * t202;
            let t204 = t203 * t80;
            let t205 = t201 * t204;
            let t208 = t173 * t178 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t194 * t205;
            let t209 = t208 * t181;
            let t210 = t191 * t208;
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t169 * t210;
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t209 * t214;
            let t218 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t169 * t215;
            let t219 = (simd::ln(t218));
            let t221 = t154 * t163 * t219;
            let t223 = -f64x8::splat(0.0310907) * t103 + t146 / f64x8::splat(2.0) + f64x8::splat(0.0098758367493069) * t149 + t221 / f64x8::splat(2.0);
            let t224 = -t130 + t146 + t182 + t221;
            let t225 = t78 * t80;
            let t226 = t81 * t16;
            let t227 = f64x8::splat(1.0) / t24;
            let t228 = (simd::cbrt(t227));
            let t230 = t225 * t226 * t228;
            let t232 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t230;
            let t233 = ((t230).sqrt());
            let t236 = ((t230) * (t230).sqrt());
            let t238 = t93 * t79;
            let t239 = t94 * t17;
            let t240 = t228 * t228;
            let t242 = t238 * t239 * t240;
            let t244 = f64x8::splat(3.79785) * t233 + f64x8::splat(0.8969) * t230 + f64x8::splat(0.204775) * t236 + f64x8::splat(0.123235) * t242;
            let t247 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t244;
            let t248 = (simd::ln(t247));
            let t250 = f64x8::splat(0.0621814) * t232 * t248;
            let t251 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t253 = ((t251).select(t108, f64x8::splat(2.0) * t16));
            let t254 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t255 = ((t254).select(t108, f64x8::splat(0.0)));
            let t257 = (t253 + t255 - f64x8::splat(2.0)) * t116;
            let t259 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t230;
            let t264 = f64x8::splat(7.05945) * t233 + f64x8::splat(1.549425) * t230 + f64x8::splat(0.420775) * t236 + f64x8::splat(0.1562925) * t242;
            let t267 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t264;
            let t268 = (simd::ln(t267));
            let t272 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t230;
            let t277 = f64x8::splat(5.1785) * t233 + f64x8::splat(0.905775) * t230 + f64x8::splat(0.1100325) * t236 + f64x8::splat(0.1241775) * t242;
            let t280 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t277;
            let t281 = (simd::ln(t280));
            let t282 = t272 * t281;
            let t285 = t257 * (-f64x8::splat(0.0310907) * t259 * t268 + t250 - f64x8::splat(0.0197516734986138) * t282);
            let t287 = f64x8::splat(0.0197516734986138) * t257 * t282;
            let t288 = ((t251).select(t155, t17));
            let t289 = ((t254).select(t155, f64x8::splat(0.0)));
            let t291 = t288 / f64x8::splat(2.0) + t289 / f64x8::splat(2.0);
            let t292 = t291 * t291;
            let t293 = t292 * t291;
            let t295 = f64x8::splat(1.0) + f64x8::splat(0.025) * t230;
            let t297 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t230;
            let t298 = f64x8::splat(1.0) / t297;
            let t299 = t295 * t298;
            let t300 = f64x8::splat(1.0) / t292;
            let t301 = t300 * t91;
            let t302 = t172 * t301;
            let t303 = f64x8::splat(1.0) / t228;
            let t304 = t17 * t303;
            let t305 = t177 * t304;
            let t308 = t299 * t181;
            let t311 = f64x8::splat(1.0) / t293;
            let t312 = t42 * t311;
            let t314 = (simd::exp(-(-t250 + t285 + t287) * t181 * t312));
            let t315 = t314 - f64x8::splat(1.0);
            let t316 = f64x8::splat(1.0) / t315;
            let t317 = t316 * t192;
            let t320 = t292 * t292;
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t321 * t41;
            let t323 = t322 * t202;
            let t324 = t80 * t16;
            let t325 = f64x8::splat(1.0) / t240;
            let t327 = t323 * t324 * t325;
            let t330 = t302 * t305 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t308 * t317 * t197 * t327;
            let t331 = t330 * t181;
            let t332 = t181 * t316;
            let t333 = t332 * t330;
            let t336 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t299 * t333;
            let t337 = f64x8::splat(1.0) / t336;
            let t338 = t331 * t337;
            let t341 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t299 * t338;
            let t342 = (simd::ln(t341));
            let t345 = t154 * t293 * t342 - t250 + t285 + t287;
            let t346 = (t224).simd_lt(t345);
            let t347 = ((t346).select(t345, t224));
            let t350 = ((t75).select(t223, t347 * t24 / f64x8::splat(2.0)));
            let t351 = f64x8::splat(1.0) / t31;
            let t352 = (simd::cbrt(t351));
            let t354 = t225 * t226 * t352;
            let t356 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t354;
            let t357 = ((t354).sqrt());
            let t360 = ((t354) * (t354).sqrt());
            let t362 = t352 * t352;
            let t364 = t238 * t239 * t362;
            let t366 = f64x8::splat(3.79785) * t357 + f64x8::splat(0.8969) * t354 + f64x8::splat(0.204775) * t360 + f64x8::splat(0.123235) * t364;
            let t369 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t366;
            let t370 = (simd::ln(t369));
            let t372 = f64x8::splat(0.0621814) * t356 * t370;
            let t374 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t354;
            let t379 = f64x8::splat(7.05945) * t357 + f64x8::splat(1.549425) * t354 + f64x8::splat(0.420775) * t360 + f64x8::splat(0.1562925) * t364;
            let t382 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t379;
            let t383 = (simd::ln(t382));
            let t387 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t354;
            let t392 = f64x8::splat(5.1785) * t357 + f64x8::splat(0.905775) * t354 + f64x8::splat(0.1100325) * t360 + f64x8::splat(0.1241775) * t364;
            let t395 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t392;
            let t396 = (simd::ln(t395));
            let t397 = t387 * t396;
            let t400 = t257 * (-f64x8::splat(0.0310907) * t374 * t383 + t372 - f64x8::splat(0.0197516734986138) * t397);
            let t402 = f64x8::splat(0.0197516734986138) * t257 * t397;
            let t404 = f64x8::splat(1.0) + f64x8::splat(0.025) * t354;
            let t406 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t354;
            let t407 = f64x8::splat(1.0) / t406;
            let t408 = t404 * t407;
            let t409 = f64x8::splat(1.0) / t352;
            let t410 = t17 * t409;
            let t411 = t177 * t410;
            let t414 = t408 * t181;
            let t418 = (simd::exp(-(-t372 + t400 + t402) * t181 * t312));
            let t419 = t418 - f64x8::splat(1.0);
            let t420 = f64x8::splat(1.0) / t419;
            let t421 = t420 * t192;
            let t424 = f64x8::splat(1.0) / t362;
            let t426 = t323 * t324 * t424;
            let t429 = t302 * t411 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t414 * t421 * t197 * t426;
            let t430 = t429 * t181;
            let t431 = t181 * t420;
            let t432 = t431 * t429;
            let t435 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t408 * t432;
            let t436 = f64x8::splat(1.0) / t435;
            let t437 = t430 * t436;
            let t440 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t408 * t437;
            let t441 = (simd::ln(t440));
            let t444 = t154 * t293 * t441 - t372 + t400 + t402;
            let t445 = (t224).simd_lt(t444);
            let t446 = ((t445).select(t444, t224));
            let t449 = ((t75).select(t223, t446 * t31 / f64x8::splat(2.0)));
            let t450 = t350 + t449;
            let t453 = t62 * t71 + f64x8::splat(1.0);
            let t454 = ((t10).select(t108, f64x8::splat(1.0)));
            let t457 = (f64x8::splat(2.0) * t454 - f64x8::splat(2.0)) * t116;
            let t459 = f64x8::splat(0.0197516734986138) * t457 * t142;
            let t460 = ((t10).select(t155, f64x8::splat(1.0)));
            let t461 = t460 * t460;
            let t462 = t461 * t460;
            let t463 = f64x8::splat(1.0) / t461;
            let t465 = t463 * t91 * t177;
            let t470 = f64x8::splat(1.0) / t462;
            let t471 = t42 * t470;
            let t473 = (simd::exp(-(-t130 + t459) * t181 * t471));
            let t474 = t473 - f64x8::splat(1.0);
            let t475 = f64x8::splat(1.0) / t474;
            let t476 = t181 * t475;
            let t477 = t476 * t192;
            let t478 = t169 * t477;
            let t479 = t461 * t461;
            let t480 = f64x8::splat(1.0) / t479;
            let t481 = t198 * t480;
            let t482 = t481 * t204;
            let t485 = t173 * t465 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t478 * t482;
            let t486 = t485 * t181;
            let t487 = t476 * t485;
            let t490 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t169 * t487;
            let t491 = f64x8::splat(1.0) / t490;
            let t492 = t486 * t491;
            let t495 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t169 * t492;
            let t496 = (simd::ln(t495));
            let t499 = t154 * t462 * t496 - t130 + t459;
            let t501 = -t72 * t450 + t453 * t499;
            let t502 = param_d * t501;
            let t503 = t71 * t70;
            let t505 = t502 * t503 + f64x8::splat(1.0);
            let tzk0 = t501 * t505;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
