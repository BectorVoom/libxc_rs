//! MGGA_C_PKZB vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_pkzb.c`
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
pub fn mgga_c_pkzb_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = v_sigma * v_sigma;
            let t3 = v_rho * v_rho;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = v_tau * v_tau;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = f64x8::splat(1.0) + f64x8::splat(0.00828125) * t5 * t7;
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(1.0) / t17;
            let t20 = t14 * t16 * t18;
            let t22 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t20;
            let t23 = ((t20).sqrt());
            let t26 = ((t20) * (t20).sqrt());
            let t28 = t11 * t11;
            let t29 = t13 * t13;
            let t30 = t28 * t29;
            let t31 = t17 * t17;
            let t32 = f64x8::splat(1.0) / t31;
            let t34 = t30 * t15 * t32;
            let t36 = f64x8::splat(3.79785) * t23 + f64x8::splat(0.8969) * t20 + f64x8::splat(0.204775) * t26 + f64x8::splat(0.123235) * t34;
            let t39 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t36;
            let t40 = (simd::ln(t39));
            let t42 = f64x8::splat(0.0621814) * t22 * t40;
            let t43 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t44 = (simd::cbrt(zeta_threshold));
            let t45 = t44 * zeta_threshold;
            let t46 = ((t43).select(t45, f64x8::splat(1.0)));
            let t49 = f64x8::splat(M_CBRT2);
            let t52 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t49 - f64x8::splat(2.0));
            let t53 = (f64x8::splat(2.0) * t46 - f64x8::splat(2.0)) * t52;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t20;
            let t60 = f64x8::splat(5.1785) * t23 + f64x8::splat(0.905775) * t20 + f64x8::splat(0.1100325) * t26 + f64x8::splat(0.1241775) * t34;
            let t63 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t60;
            let t64 = (simd::ln(t63));
            let t67 = f64x8::splat(0.0197516734986138) * t53 * t55 * t64;
            let t68 = (simd::ln(f64x8::splat(2.0)));
            let t69 = f64x8::splat(1.0) - t68;
            let t70 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t69 * t71;
            let t73 = t44 * t44;
            let t74 = ((t43).select(t73, f64x8::splat(1.0)));
            let t75 = t74 * t74;
            let t76 = t75 * t74;
            let t78 = f64x8::splat(1.0) / t17 / t3;
            let t79 = v_sigma * t78;
            let t81 = f64x8::splat(1.0) / t75;
            let t83 = f64x8::splat(1.0) / t13;
            let t84 = t83 * t15;
            let t85 = t81 * t28 * t84;
            let t88 = f64x8::splat(1.0) / t69;
            let t91 = f64x8::splat(1.0) / t76;
            let t92 = t70 * t91;
            let t94 = (simd::exp(-(-t42 + t67) * t88 * t92));
            let t95 = t94 - f64x8::splat(1.0);
            let t96 = f64x8::splat(1.0) / t95;
            let t97 = t88 * t96;
            let t98 = t3 * t3;
            let t100 = f64x8::splat(1.0) / t31 / t98;
            let t101 = t2 * t100;
            let t103 = t49 * t49;
            let t104 = t75 * t75;
            let t105 = f64x8::splat(1.0) / t104;
            let t106 = t103 * t105;
            let t107 = f64x8::splat(1.0) / t29;
            let t108 = t11 * t107;
            let t109 = t108 * t16;
            let t110 = t106 * t109;
            let t113 = t79 * t49 * t85 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t97 * t101 * t110;
            let t114 = t113 * t88;
            let t117 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t97 * t113;
            let t118 = f64x8::splat(1.0) / t117;
            let t121 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t114 * t118;
            let t122 = (simd::ln(t121));
            let t125 = t72 * t76 * t122 - t42 + t67;
            let t126 = t10 * t125;
            let t129 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t43);
            let t130 = t14 * t16;
            let t133 = ((t43).select(f64x8::splat(1.0) / t44, f64x8::splat(1.0)));
            let t135 = t130 * t18 * t49 * t133;
            let t137 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t135;
            let t138 = ((t135).sqrt());
            let t141 = ((t135) * (t135).sqrt());
            let t143 = t30 * t15;
            let t145 = t133 * t133;
            let t147 = t143 * t32 * t103 * t145;
            let t149 = f64x8::splat(3.79785) * t138 + f64x8::splat(0.8969) * t135 + f64x8::splat(0.204775) * t141 + f64x8::splat(0.123235) * t147;
            let t152 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t149;
            let t153 = (simd::ln(t152));
            let t155 = f64x8::splat(0.0621814) * t137 * t153;
            let t156 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t158 = ((t156).select(t45, f64x8::splat(2.0) * t49));
            let t159 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t160 = ((t159).select(t45, f64x8::splat(0.0)));
            let t162 = (t158 + t160 - f64x8::splat(2.0)) * t52;
            let t164 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t135;
            let t169 = f64x8::splat(7.05945) * t138 + f64x8::splat(1.549425) * t135 + f64x8::splat(0.420775) * t141 + f64x8::splat(0.1562925) * t147;
            let t172 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t169;
            let t173 = (simd::ln(t172));
            let t177 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t135;
            let t182 = f64x8::splat(5.1785) * t138 + f64x8::splat(0.905775) * t135 + f64x8::splat(0.1100325) * t141 + f64x8::splat(0.1241775) * t147;
            let t185 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t182;
            let t186 = (simd::ln(t185));
            let t187 = t177 * t186;
            let t190 = t162 * (-f64x8::splat(0.0310907) * t164 * t173 + t155 - f64x8::splat(0.0197516734986138) * t187);
            let t192 = f64x8::splat(0.0197516734986138) * t162 * t187;
            let t193 = ((t156).select(t73, t103));
            let t194 = ((t159).select(t73, f64x8::splat(0.0)));
            let t196 = t193 / f64x8::splat(2.0) + t194 / f64x8::splat(2.0);
            let t197 = t196 * t196;
            let t198 = t197 * t196;
            let t199 = f64x8::splat(1.0) / t197;
            let t200 = t199 * t28;
            let t204 = t84 * t103 / t133;
            let t209 = f64x8::splat(1.0) / t198;
            let t210 = t70 * t209;
            let t212 = (simd::exp(-(-t155 + t190 + t192) * t88 * t210));
            let t213 = t212 - f64x8::splat(1.0);
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t88 * t214;
            let t216 = t197 * t197;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = t16 * t49;
            let t221 = f64x8::splat(1.0) / t145;
            let t222 = t220 * t221;
            let t223 = t108 * t222;
            let t226 = t79 * t200 * t204 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t215 * t101 * t217 * t223;
            let t227 = t226 * t88;
            let t230 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t215 * t226;
            let t231 = f64x8::splat(1.0) / t230;
            let t234 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t227 * t231;
            let t235 = (simd::ln(t234));
            let t239 = ((t43).select(zeta_threshold, f64x8::splat(1.0)));
            let t242 = ((t129).select(f64x8::splat(0.0), (t72 * t198 * t235 - t155 + t190 + t192) * t239 / f64x8::splat(2.0)));
            let t243 = t7 * t242;
            let t245 = f64x8::splat(0.0478125) * t5 * t243;
            let tzk0 = t126 - t245;
            acc_zk = tzk0;
            let t246 = t3 * v_rho;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t2 * t247;
            let t249 = t7 * t125;
            let t250 = t248 * t249;
            let t253 = f64x8::splat(1.0) / t17 / v_rho;
            let t254 = t16 * t253;
            let t257 = f64x8::splat(0.0011073470983333333) * t14 * t254 * t40;
            let t258 = t36 * t36;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t22 * t259;
            let t262 = f64x8::splat(1.0) / t23 * t11;
            let t263 = t13 * t16;
            let t264 = t263 * t253;
            let t265 = t262 * t264;
            let t267 = t14 * t254;
            let t269 = ((t20).sqrt());
            let t270 = t269 * t11;
            let t271 = t270 * t264;
            let t274 = f64x8::splat(1.0) / t31 / v_rho;
            let t276 = t30 * t15 * t274;
            let t278 = -f64x8::splat(0.632975) * t265 - f64x8::splat(0.29896666666666666) * t267 - f64x8::splat(0.1023875) * t271 - f64x8::splat(0.08215666666666667) * t276;
            let t279 = f64x8::splat(1.0) / t39;
            let t280 = t278 * t279;
            let t282 = f64x8::splat(1.0) * t260 * t280;
            let t283 = t53 * t11;
            let t287 = f64x8::splat(0.00018311447306006544) * t283 * t263 * t253 * t64;
            let t288 = t53 * t55;
            let t289 = t60 * t60;
            let t290 = f64x8::splat(1.0) / t289;
            let t295 = -f64x8::splat(0.8630833333333333) * t265 - f64x8::splat(0.301925) * t267 - f64x8::splat(0.05501625) * t271 - f64x8::splat(0.082785) * t276;
            let t297 = f64x8::splat(1.0) / t63;
            let t298 = t290 * t295 * t297;
            let t300 = f64x8::splat(0.5848223622634646) * t288 * t298;
            let t302 = f64x8::splat(1.0) / t17 / t246;
            let t303 = v_sigma * t302;
            let t307 = t69 * t69;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t95 * t95;
            let t310 = f64x8::splat(1.0) / t309;
            let t311 = t308 * t310;
            let t312 = t311 * t2;
            let t313 = t100 * t103;
            let t315 = f64x8::splat(1.0) / t104 / t76;
            let t316 = t313 * t315;
            let t317 = t312 * t316;
            let t318 = t257 + t282 - t287 - t300;
            let t319 = t318 * t70;
            let t320 = t319 * t94;
            let t321 = t109 * t320;
            let t324 = t98 * v_rho;
            let t326 = f64x8::splat(1.0) / t31 / t324;
            let t327 = t2 * t326;
            let t331 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t303 * t49 * t85 + f64x8::splat(0.0002143700905903487) * t317 * t321 - f64x8::splat(0.0010003937560882938) * t97 * t327 * t110;
            let t332 = t331 * t88;
            let t335 = t117 * t117;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t311 * t113;
            let t338 = t91 * t94;
            let t339 = t319 * t338;
            let t344 = f64x8::splat(0.6585449182935511) * t337 * t339 + f64x8::splat(0.6585449182935511) * t97 * t331;
            let t345 = t336 * t344;
            let t348 = f64x8::splat(0.6585449182935511) * t332 * t118 - f64x8::splat(0.6585449182935511) * t114 * t345;
            let t350 = f64x8::splat(1.0) / t121;
            let t353 = t72 * t76 * t348 * t350 + t257 + t282 - t287 - t300;
            let t354 = t10 * t353;
            let t355 = t248 * t243;
            let t357 = t253 * t49;
            let t358 = t133 * t153;
            let t361 = f64x8::splat(0.0011073470983333333) * t130 * t357 * t358;
            let t362 = t149 * t149;
            let t363 = f64x8::splat(1.0) / t362;
            let t364 = t137 * t363;
            let t367 = f64x8::splat(1.0) / t138 * t11 * t13;
            let t368 = t49 * t133;
            let t369 = t254 * t368;
            let t370 = t367 * t369;
            let t372 = t357 * t133;
            let t373 = t130 * t372;
            let t375 = ((t135).sqrt());
            let t377 = t375 * t11 * t13;
            let t378 = t377 * t369;
            let t382 = t143 * t274 * t103 * t145;
            let t384 = -f64x8::splat(0.632975) * t370 - f64x8::splat(0.29896666666666666) * t373 - f64x8::splat(0.1023875) * t378 - f64x8::splat(0.08215666666666667) * t382;
            let t385 = f64x8::splat(1.0) / t152;
            let t386 = t384 * t385;
            let t388 = f64x8::splat(1.0) * t364 * t386;
            let t389 = t133 * t173;
            let t393 = t169 * t169;
            let t394 = f64x8::splat(1.0) / t393;
            let t395 = t164 * t394;
            let t400 = -f64x8::splat(1.176575) * t370 - f64x8::splat(0.516475) * t373 - f64x8::splat(0.2103875) * t378 - f64x8::splat(0.104195) * t382;
            let t401 = f64x8::splat(1.0) / t172;
            let t402 = t400 * t401;
            let t405 = t133 * t186;
            let t409 = t182 * t182;
            let t410 = f64x8::splat(1.0) / t409;
            let t411 = t177 * t410;
            let t416 = -f64x8::splat(0.8630833333333333) * t370 - f64x8::splat(0.301925) * t373 - f64x8::splat(0.05501625) * t378 - f64x8::splat(0.082785) * t382;
            let t417 = f64x8::splat(1.0) / t185;
            let t418 = t416 * t417;
            let t422 = t162 * (f64x8::splat(0.0005323764196666666) * t130 * t357 * t389 + f64x8::splat(1.0) * t395 * t402 - t361 - t388 + f64x8::splat(0.00018311447306006544) * t130 * t357 * t405 + f64x8::splat(0.5848223622634646) * t411 * t418);
            let t423 = t162 * t14;
            let t424 = t368 * t186;
            let t427 = f64x8::splat(0.00018311447306006544) * t423 * t254 * t424;
            let t428 = t162 * t177;
            let t430 = t410 * t416 * t417;
            let t432 = f64x8::splat(0.5848223622634646) * t428 * t430;
            let t436 = t213 * t213;
            let t437 = f64x8::splat(1.0) / t436;
            let t438 = t308 * t437;
            let t439 = t438 * t2;
            let t441 = f64x8::splat(1.0) / t216 / t198;
            let t442 = t100 * t441;
            let t443 = t442 * t11;
            let t444 = t439 * t443;
            let t445 = t107 * t16;
            let t446 = t445 * t49;
            let t447 = t361 + t388 + t422 - t427 - t432;
            let t449 = t70 * t212;
            let t450 = t221 * t447 * t449;
            let t451 = t446 * t450;
            let t458 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t303 * t200 * t204 + f64x8::splat(0.0004287401811806974) * t444 * t451 - f64x8::splat(0.0020007875121765876) * t215 * t327 * t217 * t223;
            let t459 = t458 * t88;
            let t462 = t230 * t230;
            let t463 = f64x8::splat(1.0) / t462;
            let t464 = t438 * t226;
            let t465 = t447 * t70;
            let t466 = t209 * t212;
            let t467 = t465 * t466;
            let t472 = f64x8::splat(0.6585449182935511) * t464 * t467 + f64x8::splat(0.6585449182935511) * t215 * t458;
            let t473 = t463 * t472;
            let t476 = f64x8::splat(0.6585449182935511) * t459 * t231 - f64x8::splat(0.6585449182935511) * t227 * t473;
            let t478 = f64x8::splat(1.0) / t234;
            let t484 = ((t129).select(f64x8::splat(0.0), (t72 * t198 * t476 * t478 + t361 + t388 + t422 - t427 - t432) * t239 / f64x8::splat(2.0)));
            let t485 = t7 * t484;
            let t486 = t5 * t485;
            let tvrho0 = t126 - t245 + v_rho * (-f64x8::splat(0.0165625) * t250 + t354 + f64x8::splat(0.095625) * t355 - f64x8::splat(0.0478125) * t486);
            acc_vrho = tvrho0;
            let t490 = v_sigma * t4;
            let t492 = f64x8::splat(0.0165625) * t490 * t249;
            let t493 = t10 * t69;
            let t494 = t493 * t71;
            let t495 = t78 * t49;
            let t498 = t28 * t83 * t15;
            let t501 = v_sigma * t100;
            let t505 = t495 * t81 * t498 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t97 * t501 * t110;
            let t506 = t505 * t88;
            let t509 = t113 * t308;
            let t510 = t336 * t96;
            let t511 = t510 * t505;
            let t514 = f64x8::splat(0.6585449182935511) * t506 * t118 - f64x8::splat(0.43368140941025995) * t509 * t511;
            let t515 = t76 * t514;
            let t516 = t515 * t350;
            let t517 = t494 * t516;
            let t519 = f64x8::splat(0.095625) * t490 * t243;
            let t520 = t72 * t198;
            let t529 = t78 * t199 * t28 * t204 / f64x8::splat(96.0) + f64x8::splat(0.0008574803623613948) * t215 * t501 * t217 * t223;
            let t530 = t529 * t88;
            let t533 = t226 * t308;
            let t534 = t463 * t214;
            let t535 = t534 * t529;
            let t538 = f64x8::splat(0.6585449182935511) * t530 * t231 - f64x8::splat(0.43368140941025995) * t533 * t535;
            let t543 = ((t129).select(f64x8::splat(0.0), t520 * t538 * t478 * t239 / f64x8::splat(2.0)));
            let t544 = t7 * t543;
            let t546 = f64x8::splat(0.0478125) * t5 * t544;
            let tvsigma0 = v_rho * (t492 + t517 - t519 - t546);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t549 = f64x8::splat(1.0) / t6 / v_tau;
            let t550 = t549 * t125;
            let t552 = f64x8::splat(0.0165625) * t5 * t550;
            let t553 = t549 * t242;
            let t555 = f64x8::splat(0.095625) * t5 * t553;
            let tvtau0 = v_rho * (-t552 + t555);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
