//! MGGA_C_TPSSLOC exc unpol kernel — explicit SIMD.
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

// `exp`, `ln` and the cube-root family come from `libxc_rkernel_math::simd`,
// which is bit-identical per lane to the scalar calls the scalar kernel makes
// (exp/ln to glibc's `_fma` ifuncs, cbrt to `powers::cbrt_f64`). Only
// `atan`/`tanh`-class calls still use `wide`'s ~1 ulp forms; a kernel with
// none of those produces output bit-identical to its scalar form.

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
pub fn mgga_c_tpssloc_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (((f64x8::splat(0.0)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t4 = (-t3).simd_le(-f64x8::splat(0.999999999999));
            let t5 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t6 = zeta_threshold - f64x8::splat(1.0);
            let t8 = ((t5).select(t6, (t5).select(-t6, f64x8::splat(0.0))));
            let t9 = t8 * t8;
            let t10 = f64x8::splat(1.0) - t9;
            let t11 = f64x8::splat(M_CBRT2);
            let t12 = t11 * t11;
            let t13 = v_sigma * t12;
            let t14 = v_rho * v_rho;
            let t15 = (simd::cbrt(v_rho));
            let t16 = t15 * t15;
            let t18 = f64x8::splat(1.0) / t16 / t14;
            let t19 = f64x8::splat(1.0) + t8;
            let t20 = t19 / f64x8::splat(2.0);
            let t21 = (simd::cbrt(t20));
            let t22 = t21 * t21;
            let t23 = t22 * t20;
            let t26 = f64x8::splat(1.0) - t8;
            let t27 = t26 / f64x8::splat(2.0);
            let t28 = (simd::cbrt(t27));
            let t29 = t28 * t28;
            let t30 = t29 * t27;
            let t36 = f64x8::splat(M_CBRT3);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t41 = t36 / t39;
            let t42 = (simd::cbrt(t19));
            let t43 = t42 * t19;
            let t45 = (simd::cbrt(t26));
            let t46 = t45 * t26;
            let t48 = f64x8::splat(1.0) / t43 + f64x8::splat(1.0) / t46;
            let t49 = t41 * t48;
            let t52 = f64x8::splat(1.0) + t10 * (t13 * t18 * t23 + t13 * t18 * t30 - v_sigma * t18) * t49 / f64x8::splat(24.0);
            let t53 = t52 * t52;
            let t54 = t53 * t53;
            let t57 = ((t4).select(f64x8::splat(3.98), f64x8::splat(0.35) / t54));
            let t58 = f64x8::splat(1.0) + t57;
            let t59 = f64x8::splat(1.0) / v_rho;
            let t60 = v_sigma * t59;
            let t61 = f64x8::splat(1.0) / v_tau;
            let t63 = t60 * t61 / f64x8::splat(8.0);
            let t64 = (f64x8::splat(1.0)).simd_lt(t63);
            let t65 = ((t64).select(f64x8::splat(1.0), t63));
            let t66 = t65 * t65;
            let t67 = t58 * t66;
            let t70 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t5);
            let t71 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t72 = (simd::cbrt(t71));
            let t73 = t36 * t72;
            let t74 = f64x8::splat(M_CBRT4);
            let t75 = t74 * t74;
            let t76 = f64x8::splat(1.0) / t15;
            let t78 = t73 * t75 * t76;
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t78;
            let t81 = ((t78).sqrt());
            let t84 = ((t78) * (t78).sqrt());
            let t86 = t36 * t36;
            let t87 = t72 * t72;
            let t88 = t86 * t87;
            let t89 = f64x8::splat(1.0) / t16;
            let t91 = t88 * t74 * t89;
            let t93 = f64x8::splat(3.79785) * t81 + f64x8::splat(0.8969) * t78 + f64x8::splat(0.204775) * t84 + f64x8::splat(0.123235) * t91;
            let t96 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t93;
            let t97 = (simd::ln(t96));
            let t98 = t80 * t97;
            let t100 = t9 * t9;
            let t101 = (t19).simd_le(zeta_threshold);
            let t102 = (simd::cbrt(zeta_threshold));
            let t103 = t102 * zeta_threshold;
            let t104 = ((t101).select(t103, t43));
            let t105 = (t26).simd_le(zeta_threshold);
            let t106 = ((t105).select(t103, t46));
            let t107 = t104 + t106 - f64x8::splat(2.0);
            let t108 = t100 * t107;
            let t111 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t11 - f64x8::splat(2.0));
            let t113 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t78;
            let t118 = f64x8::splat(7.05945) * t81 + f64x8::splat(1.549425) * t78 + f64x8::splat(0.420775) * t84 + f64x8::splat(0.1562925) * t91;
            let t121 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t118;
            let t122 = (simd::ln(t121));
            let t125 = f64x8::splat(0.0621814) * t98;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t78;
            let t132 = f64x8::splat(5.1785) * t81 + f64x8::splat(0.905775) * t78 + f64x8::splat(0.1100325) * t84 + f64x8::splat(0.1241775) * t91;
            let t135 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t132;
            let t136 = (simd::ln(t135));
            let t137 = t127 * t136;
            let t141 = t108 * t111 * (-f64x8::splat(0.0310907) * t113 * t122 + t125 - f64x8::splat(0.0197516734986138) * t137);
            let t143 = t107 * t111;
            let t144 = t143 * t137;
            let t146 = (simd::ln(f64x8::splat(2.0)));
            let t147 = f64x8::splat(1.0) - t146;
            let t148 = f64x8::splat(1.0) / t37;
            let t149 = t147 * t148;
            let t150 = t102 * t102;
            let t151 = t42 * t42;
            let t152 = ((t101).select(t150, t151));
            let t153 = t45 * t45;
            let t154 = ((t105).select(t150, t153));
            let t156 = t152 / f64x8::splat(2.0) + t154 / f64x8::splat(2.0);
            let t157 = t156 * t156;
            let t158 = t157 * t156;
            let t160 = f64x8::splat(1.0) / t15 / t14;
            let t161 = v_sigma * t160;
            let t162 = f64x8::splat(1.0) / t157;
            let t163 = t11 * t162;
            let t165 = f64x8::splat(1.0) / t72;
            let t166 = t86 * t165;
            let t168 = (simd::exp(-t91 / f64x8::splat(4.0)));
            let t169 = f64x8::splat(1.0) - t168;
            let t170 = t74 * t169;
            let t171 = t166 * t170;
            let t174 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t161 * t163 * t171;
            let t175 = t161 * t11;
            let t176 = t162 * t86;
            let t177 = t165 * t74;
            let t178 = t176 * t177;
            let t181 = f64x8::splat(1.0) / t147;
            let t182 = t174 * t181;
            let t183 = f64x8::splat(0.0197516734986138) * t144;
            let t186 = f64x8::splat(1.0) / t158;
            let t189 = (simd::exp(-(-t125 + t141 + t183) * t181 * t37 * t186));
            let t190 = t189 - f64x8::splat(1.0);
            let t191 = f64x8::splat(1.0) / t190;
            let t192 = t37 * t191;
            let t193 = v_sigma * v_sigma;
            let t194 = t192 * t193;
            let t195 = t182 * t194;
            let t196 = t14 * t14;
            let t198 = f64x8::splat(1.0) / t16 / t196;
            let t199 = t198 * t12;
            let t200 = t157 * t157;
            let t201 = f64x8::splat(1.0) / t200;
            let t203 = f64x8::splat(1.0) / t87;
            let t204 = t36 * t203;
            let t205 = t204 * t75;
            let t206 = t199 * t201 * t205;
            let t209 = t175 * t178 / f64x8::splat(96.0) + t195 * t206 / f64x8::splat(3072.0);
            let t210 = t174 * t209;
            let t211 = t181 * t37;
            let t212 = t192 * t209;
            let t214 = t182 * t212 + f64x8::splat(1.0);
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = t211 * t215;
            let t218 = t210 * t216 + f64x8::splat(1.0);
            let t219 = (simd::ln(t218));
            let t221 = t149 * t158 * t219;
            let t223 = -f64x8::splat(0.0310907) * t98 + t141 / f64x8::splat(2.0) + f64x8::splat(0.0098758367493069) * t144 + t221 / f64x8::splat(2.0);
            let t224 = -t125 + t141 + t183 + t221;
            let t225 = t73 * t75;
            let t226 = t76 * t11;
            let t227 = f64x8::splat(1.0) / t19;
            let t228 = (simd::cbrt(t227));
            let t230 = t225 * t226 * t228;
            let t232 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t230;
            let t233 = ((t230).sqrt());
            let t236 = ((t230) * (t230).sqrt());
            let t238 = t88 * t74;
            let t239 = t89 * t12;
            let t240 = t228 * t228;
            let t242 = t238 * t239 * t240;
            let t244 = f64x8::splat(3.79785) * t233 + f64x8::splat(0.8969) * t230 + f64x8::splat(0.204775) * t236 + f64x8::splat(0.123235) * t242;
            let t247 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t244;
            let t248 = (simd::ln(t247));
            let t250 = f64x8::splat(0.0621814) * t232 * t248;
            let t251 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t253 = ((t251).select(t103, f64x8::splat(2.0) * t11));
            let t254 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t255 = ((t254).select(t103, f64x8::splat(0.0)));
            let t257 = (t253 + t255 - f64x8::splat(2.0)) * t111;
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
            let t288 = ((t251).select(t150, t12));
            let t289 = ((t254).select(t150, f64x8::splat(0.0)));
            let t291 = t288 / f64x8::splat(2.0) + t289 / f64x8::splat(2.0);
            let t292 = t291 * t291;
            let t293 = t292 * t291;
            let t294 = f64x8::splat(1.0) / t292;
            let t295 = t294 * t86;
            let t296 = t161 * t295;
            let t297 = f64x8::splat(1.0) / t228;
            let t298 = t12 * t297;
            let t300 = (simd::exp(-t242 / f64x8::splat(4.0)));
            let t301 = f64x8::splat(1.0) - t300;
            let t302 = t298 * t301;
            let t303 = t177 * t302;
            let t306 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t296 * t303;
            let t307 = t177 * t298;
            let t310 = t306 * t181;
            let t311 = t310 * t37;
            let t314 = f64x8::splat(1.0) / t293;
            let t315 = t37 * t314;
            let t317 = (simd::exp(-(-t250 + t285 + t287) * t181 * t315));
            let t318 = t317 - f64x8::splat(1.0);
            let t319 = f64x8::splat(1.0) / t318;
            let t320 = t319 * t193;
            let t321 = t320 * t198;
            let t323 = t292 * t292;
            let t326 = f64x8::splat(1.0) / t323 * t36 * t203;
            let t327 = t75 * t11;
            let t328 = f64x8::splat(1.0) / t240;
            let t330 = t326 * t327 * t328;
            let t333 = t296 * t307 / f64x8::splat(96.0) + t311 * t321 * t330 / f64x8::splat(1536.0);
            let t334 = t306 * t333;
            let t335 = t37 * t319;
            let t336 = t335 * t333;
            let t338 = t310 * t336 + f64x8::splat(1.0);
            let t339 = f64x8::splat(1.0) / t338;
            let t340 = t211 * t339;
            let t342 = t334 * t340 + f64x8::splat(1.0);
            let t343 = (simd::ln(t342));
            let t346 = t149 * t293 * t343 - t250 + t285 + t287;
            let t347 = (t224).simd_lt(t346);
            let t348 = ((t347).select(t346, t224));
            let t351 = ((t70).select(t223, t348 * t19 / f64x8::splat(2.0)));
            let t352 = f64x8::splat(1.0) / t26;
            let t353 = (simd::cbrt(t352));
            let t355 = t225 * t226 * t353;
            let t357 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t355;
            let t358 = ((t355).sqrt());
            let t361 = ((t355) * (t355).sqrt());
            let t363 = t353 * t353;
            let t365 = t238 * t239 * t363;
            let t367 = f64x8::splat(3.79785) * t358 + f64x8::splat(0.8969) * t355 + f64x8::splat(0.204775) * t361 + f64x8::splat(0.123235) * t365;
            let t370 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t367;
            let t371 = (simd::ln(t370));
            let t373 = f64x8::splat(0.0621814) * t357 * t371;
            let t375 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t355;
            let t380 = f64x8::splat(7.05945) * t358 + f64x8::splat(1.549425) * t355 + f64x8::splat(0.420775) * t361 + f64x8::splat(0.1562925) * t365;
            let t383 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t380;
            let t384 = (simd::ln(t383));
            let t388 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t355;
            let t393 = f64x8::splat(5.1785) * t358 + f64x8::splat(0.905775) * t355 + f64x8::splat(0.1100325) * t361 + f64x8::splat(0.1241775) * t365;
            let t396 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t393;
            let t397 = (simd::ln(t396));
            let t398 = t388 * t397;
            let t401 = t257 * (-f64x8::splat(0.0310907) * t375 * t384 + t373 - f64x8::splat(0.0197516734986138) * t398);
            let t403 = f64x8::splat(0.0197516734986138) * t257 * t398;
            let t404 = f64x8::splat(1.0) / t353;
            let t405 = t12 * t404;
            let t407 = (simd::exp(-t365 / f64x8::splat(4.0)));
            let t408 = f64x8::splat(1.0) - t407;
            let t409 = t405 * t408;
            let t410 = t177 * t409;
            let t413 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t296 * t410;
            let t414 = t177 * t405;
            let t417 = t413 * t181;
            let t418 = t417 * t37;
            let t422 = (simd::exp(-(-t373 + t401 + t403) * t181 * t315));
            let t423 = t422 - f64x8::splat(1.0);
            let t424 = f64x8::splat(1.0) / t423;
            let t425 = t424 * t193;
            let t426 = t425 * t198;
            let t428 = f64x8::splat(1.0) / t363;
            let t430 = t326 * t327 * t428;
            let t433 = t296 * t414 / f64x8::splat(96.0) + t418 * t426 * t430 / f64x8::splat(1536.0);
            let t434 = t413 * t433;
            let t435 = t37 * t424;
            let t436 = t435 * t433;
            let t438 = t417 * t436 + f64x8::splat(1.0);
            let t439 = f64x8::splat(1.0) / t438;
            let t440 = t211 * t439;
            let t442 = t434 * t440 + f64x8::splat(1.0);
            let t443 = (simd::ln(t442));
            let t446 = t149 * t293 * t443 - t373 + t401 + t403;
            let t447 = (t224).simd_lt(t446);
            let t448 = ((t447).select(t446, t224));
            let t451 = ((t70).select(t223, t448 * t26 / f64x8::splat(2.0)));
            let t452 = t351 + t451;
            let t455 = t57 * t66 + f64x8::splat(1.0);
            let t456 = ((t5).select(t103, f64x8::splat(1.0)));
            let t459 = (f64x8::splat(2.0) * t456 - f64x8::splat(2.0)) * t111;
            let t461 = f64x8::splat(0.0197516734986138) * t459 * t137;
            let t462 = ((t5).select(t150, f64x8::splat(1.0)));
            let t463 = t462 * t462;
            let t464 = t463 * t462;
            let t465 = f64x8::splat(1.0) / t463;
            let t466 = t11 * t465;
            let t470 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t161 * t466 * t171;
            let t471 = t465 * t86;
            let t472 = t471 * t177;
            let t475 = t470 * t181;
            let t478 = f64x8::splat(1.0) / t464;
            let t481 = (simd::exp(-(-t125 + t461) * t181 * t37 * t478));
            let t482 = t481 - f64x8::splat(1.0);
            let t483 = f64x8::splat(1.0) / t482;
            let t484 = t37 * t483;
            let t485 = t484 * t193;
            let t486 = t475 * t485;
            let t487 = t463 * t463;
            let t488 = f64x8::splat(1.0) / t487;
            let t490 = t199 * t488 * t205;
            let t493 = t175 * t472 / f64x8::splat(96.0) + t486 * t490 / f64x8::splat(3072.0);
            let t494 = t470 * t493;
            let t495 = t484 * t493;
            let t497 = t475 * t495 + f64x8::splat(1.0);
            let t498 = f64x8::splat(1.0) / t497;
            let t499 = t211 * t498;
            let t501 = t494 * t499 + f64x8::splat(1.0);
            let t502 = (simd::ln(t501));
            let t505 = t149 * t464 * t502 - t125 + t461;
            let t507 = -t67 * t452 + t455 * t505;
            let t508 = t66 * t65;
            let t511 = f64x8::splat(1.0) + f64x8::splat(4.5) * t507 * t508;
            let tzk0 = t507 * t511;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
