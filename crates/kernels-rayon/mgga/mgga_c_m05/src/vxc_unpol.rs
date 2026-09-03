//! MGGA_C_M05 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m05.c`
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
pub fn mgga_c_m05_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_css_1: f64,
    param_gamma_ss: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_css_0: f64,
    param_Fermi_D_cnst: f64,
    param_cab_1: f64,
    param_gamma_ab: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_cab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_css_1 = f64x8::splat(param_css_1);
    let param_gamma_ss = f64x8::splat(param_gamma_ss);
    let param_css_2 = f64x8::splat(param_css_2);
    let param_css_3 = f64x8::splat(param_css_3);
    let param_css_4 = f64x8::splat(param_css_4);
    let param_css_0 = f64x8::splat(param_css_0);
    let param_Fermi_D_cnst = f64x8::splat(param_Fermi_D_cnst);
    let param_cab_1 = f64x8::splat(param_cab_1);
    let param_gamma_ab = f64x8::splat(param_gamma_ab);
    let param_cab_2 = f64x8::splat(param_cab_2);
    let param_cab_3 = f64x8::splat(param_cab_3);
    let param_cab_4 = f64x8::splat(param_cab_4);
    let param_cab_0 = f64x8::splat(param_cab_0);
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
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t4);
            let t6 = ((t4).select(zeta_threshold, f64x8::splat(1.0)));
            let t7 = f64x8::splat(M_CBRT3);
            let t8 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t9 = (simd::cbrt(t8));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = t10 * t12;
            let t14 = (simd::cbrt(v_rho));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = f64x8::splat(M_CBRT2);
            let t18 = (simd::cbrt(zeta_threshold));
            let t20 = ((t4).select(f64x8::splat(1.0) / t18, f64x8::splat(1.0)));
            let t22 = t13 * t15 * t16 * t20;
            let t24 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t22;
            let t25 = ((t22).sqrt());
            let t28 = ((t22) * (t22).sqrt());
            let t30 = t7 * t7;
            let t31 = t9 * t9;
            let t32 = t30 * t31;
            let t33 = t32 * t11;
            let t34 = t14 * t14;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t16 * t16;
            let t38 = t20 * t20;
            let t40 = t33 * t35 * t36 * t38;
            let t42 = f64x8::splat(3.79785) * t25 + f64x8::splat(0.8969) * t22 + f64x8::splat(0.204775) * t28 + f64x8::splat(0.123235) * t40;
            let t45 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t42;
            let t46 = (simd::ln(t45));
            let t48 = f64x8::splat(0.0621814) * t24 * t46;
            let t50 = t18 * zeta_threshold;
            let t52 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(2.0) * t16));
            let t54 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(0.0)));
            let t58 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t16 - f64x8::splat(2.0));
            let t59 = (t52 + t54 - f64x8::splat(2.0)) * t58;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t22;
            let t66 = f64x8::splat(7.05945) * t25 + f64x8::splat(1.549425) * t22 + f64x8::splat(0.420775) * t28 + f64x8::splat(0.1562925) * t40;
            let t69 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t66;
            let t70 = (simd::ln(t69));
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t22;
            let t79 = f64x8::splat(5.1785) * t25 + f64x8::splat(0.905775) * t22 + f64x8::splat(0.1100325) * t28 + f64x8::splat(0.1241775) * t40;
            let t82 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t79;
            let t83 = (simd::ln(t82));
            let t84 = t74 * t83;
            let t93 = ((t5).select(f64x8::splat(0.0), t6 * (-t48 + t59 * (-f64x8::splat(0.0310907) * t61 * t70 + t48 - f64x8::splat(0.0197516734986138) * t84) + f64x8::splat(0.0197516734986138) * t59 * t84) / f64x8::splat(2.0)));
            let t95 = param_css_1;
            let t96 = t95 * param_gamma_ss;
            let t97 = t96 * v_sigma;
            let t98 = v_rho * v_rho;
            let t100 = f64x8::splat(1.0) / t34 / t98;
            let t101 = t36 * t100;
            let t104 = param_gamma_ss * v_sigma * t101 + f64x8::splat(1.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t106 = t101 * t105;
            let t108 = param_css_2;
            let t109 = param_gamma_ss * param_gamma_ss;
            let t110 = t108 * t109;
            let t111 = v_sigma * v_sigma;
            let t112 = t110 * t111;
            let t113 = t98 * t98;
            let t114 = t113 * v_rho;
            let t116 = f64x8::splat(1.0) / t14 / t114;
            let t117 = t16 * t116;
            let t118 = t104 * t104;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t123 = param_css_3;
            let t124 = t109 * param_gamma_ss;
            let t125 = t123 * t124;
            let t126 = t111 * v_sigma;
            let t127 = t113 * t113;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t126 * t128;
            let t130 = t118 * t104;
            let t131 = f64x8::splat(1.0) / t130;
            let t135 = param_css_4;
            let t136 = t109 * t109;
            let t137 = t135 * t136;
            let t138 = t111 * t111;
            let t139 = t137 * t138;
            let t140 = t127 * t98;
            let t142 = f64x8::splat(1.0) / t34 / t140;
            let t143 = t36 * t142;
            let t144 = t118 * t118;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t143 * t145;
            let t149 = f64x8::splat(4.0) * t125 * t129 * t131 + t97 * t106 + f64x8::splat(2.0) * t112 * t120 + f64x8::splat(4.0) * t139 * t146 + param_css_0;
            let t150 = t93 * t149;
            let t151 = f64x8::splat(1.0) / v_rho;
            let t153 = f64x8::splat(1.0) / v_tau;
            let t156 = f64x8::splat(1.0) - v_sigma * t151 * t153 / f64x8::splat(8.0);
            let t157 = v_tau * v_tau;
            let t159 = t98 * v_rho;
            let t161 = f64x8::splat(1.0) / t14 / t159;
            let t162 = param_Fermi_D_cnst * param_Fermi_D_cnst;
            let t163 = f64x8::splat(1.0) / t162;
            let t167 = (simd::exp(-f64x8::splat(8.0) * t157 * t16 * t161 * t163));
            let t168 = f64x8::splat(1.0) - t167;
            let t169 = t156 * t168;
            let t171 = f64x8::splat(2.0) * t150 * t169;
            let t173 = t10 * t12 * t15;
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t173;
            let t176 = ((t173).sqrt());
            let t179 = ((t173) * (t173).sqrt());
            let t182 = t32 * t11 * t35;
            let t184 = f64x8::splat(3.79785) * t176 + f64x8::splat(0.8969) * t173 + f64x8::splat(0.204775) * t179 + f64x8::splat(0.123235) * t182;
            let t187 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t184;
            let t188 = (simd::ln(t187));
            let t191 = ((t4).select(t50, f64x8::splat(1.0)));
            let t194 = (f64x8::splat(2.0) * t191 - f64x8::splat(2.0)) * t58;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t173;
            let t201 = f64x8::splat(5.1785) * t176 + f64x8::splat(0.905775) * t173 + f64x8::splat(0.1100325) * t179 + f64x8::splat(0.1241775) * t182;
            let t204 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t201;
            let t205 = (simd::ln(t204));
            let t210 = -f64x8::splat(0.0621814) * t175 * t188 + f64x8::splat(0.0197516734986138) * t194 * t196 * t205 - f64x8::splat(2.0) * t93;
            let t212 = param_cab_1;
            let t213 = t212 * param_gamma_ab;
            let t214 = t213 * v_sigma;
            let t218 = f64x8::splat(2.0) * param_gamma_ab * v_sigma * t101 + f64x8::splat(1.0);
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t101 * t219;
            let t223 = param_cab_2;
            let t224 = param_gamma_ab * param_gamma_ab;
            let t225 = t223 * t224;
            let t226 = t225 * t111;
            let t227 = t218 * t218;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t117 * t228;
            let t232 = param_cab_3;
            let t233 = t224 * param_gamma_ab;
            let t234 = t232 * t233;
            let t235 = t227 * t218;
            let t236 = f64x8::splat(1.0) / t235;
            let t240 = param_cab_4;
            let t241 = t224 * t224;
            let t242 = t240 * t241;
            let t243 = t242 * t138;
            let t244 = t227 * t227;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t143 * t245;
            let t249 = f64x8::splat(32.0) * t234 * t129 * t236 + f64x8::splat(2.0) * t214 * t220 + f64x8::splat(8.0) * t226 * t229 + f64x8::splat(64.0) * t243 * t246 + param_cab_0;
            let t250 = t210 * t249;
            let tzk0 = t171 + t250;
            acc_zk = tzk0;
            let t252 = f64x8::splat(1.0) / t14 / v_rho;
            let t253 = t252 * t16;
            let t254 = t20 * t46;
            let t257 = f64x8::splat(0.0011073470983333333) * t13 * t253 * t254;
            let t258 = t42 * t42;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t24 * t259;
            let t263 = f64x8::splat(1.0) / t25 * t7 * t9;
            let t264 = t12 * t252;
            let t265 = t16 * t20;
            let t266 = t264 * t265;
            let t267 = t263 * t266;
            let t269 = t253 * t20;
            let t270 = t13 * t269;
            let t272 = ((t22).sqrt());
            let t274 = t272 * t7 * t9;
            let t275 = t274 * t266;
            let t278 = f64x8::splat(1.0) / t34 / v_rho;
            let t281 = t33 * t278 * t36 * t38;
            let t283 = -f64x8::splat(0.632975) * t267 - f64x8::splat(0.29896666666666666) * t270 - f64x8::splat(0.1023875) * t275 - f64x8::splat(0.08215666666666667) * t281;
            let t284 = f64x8::splat(1.0) / t45;
            let t285 = t283 * t284;
            let t287 = f64x8::splat(1.0) * t260 * t285;
            let t288 = t20 * t70;
            let t292 = t66 * t66;
            let t293 = f64x8::splat(1.0) / t292;
            let t294 = t61 * t293;
            let t299 = -f64x8::splat(1.176575) * t267 - f64x8::splat(0.516475) * t270 - f64x8::splat(0.2103875) * t275 - f64x8::splat(0.104195) * t281;
            let t300 = f64x8::splat(1.0) / t69;
            let t301 = t299 * t300;
            let t304 = t20 * t83;
            let t308 = t79 * t79;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t74 * t309;
            let t315 = -f64x8::splat(0.8630833333333333) * t267 - f64x8::splat(0.301925) * t270 - f64x8::splat(0.05501625) * t275 - f64x8::splat(0.082785) * t281;
            let t316 = f64x8::splat(1.0) / t82;
            let t317 = t315 * t316;
            let t322 = t59 * t10;
            let t323 = t265 * t83;
            let t327 = t59 * t74;
            let t329 = t309 * t315 * t316;
            let t335 = ((t5).select(f64x8::splat(0.0), t6 * (t257 + t287 + t59 * (f64x8::splat(0.0005323764196666666) * t13 * t253 * t288 + f64x8::splat(1.0) * t294 * t301 - t257 - t287 + f64x8::splat(0.00018311447306006544) * t13 * t253 * t304 + f64x8::splat(0.5848223622634646) * t310 * t317) - f64x8::splat(0.00018311447306006544) * t322 * t264 * t323 - f64x8::splat(0.5848223622634646) * t327 * t329) / f64x8::splat(2.0)));
            let t336 = t335 * t149;
            let t337 = t336 * t169;
            let t340 = f64x8::splat(1.0) / t34 / t159;
            let t341 = t36 * t340;
            let t342 = t341 * t105;
            let t345 = t95 * t109;
            let t346 = t345 * t111;
            let t347 = t113 * t98;
            let t349 = f64x8::splat(1.0) / t14 / t347;
            let t350 = t16 * t349;
            let t351 = t350 * t119;
            let t356 = t108 * t124;
            let t357 = t127 * v_rho;
            let t358 = f64x8::splat(1.0) / t357;
            let t359 = t126 * t358;
            let t360 = t359 * t131;
            let t365 = t123 * t136;
            let t366 = t365 * t138;
            let t367 = t127 * t159;
            let t369 = f64x8::splat(1.0) / t34 / t367;
            let t371 = t369 * t145 * t36;
            let t376 = t136 * param_gamma_ss;
            let t377 = t135 * t376;
            let t378 = t138 * v_sigma;
            let t379 = t377 * t378;
            let t380 = t127 * t347;
            let t382 = f64x8::splat(1.0) / t14 / t380;
            let t383 = t16 * t382;
            let t385 = f64x8::splat(1.0) / t144 / t104;
            let t386 = t383 * t385;
            let t389 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t342 + f64x8::splat(16.0) / f64x8::splat(3.0) * t346 * t351 - f64x8::splat(32.0) / f64x8::splat(3.0) * t112 * t351 + f64x8::splat(64.0) / f64x8::splat(3.0) * t356 * t360 - f64x8::splat(32.0) * t125 * t360 + f64x8::splat(32.0) * t366 * t371 - f64x8::splat(128.0) / f64x8::splat(3.0) * t139 * t371 + f64x8::splat(256.0) / f64x8::splat(3.0) * t379 * t386;
            let t390 = t93 * t389;
            let t391 = t390 * t169;
            let t393 = t150 * v_sigma;
            let t394 = f64x8::splat(1.0) / t98;
            let t396 = t394 * t153 * t168;
            let t397 = t393 * t396;
            let t399 = t156 * t157;
            let t400 = t150 * t399;
            let t402 = f64x8::splat(1.0) / t14 / t113;
            let t403 = t16 * t402;
            let t404 = t163 * t167;
            let t405 = t403 * t404;
            let t406 = t400 * t405;
            let t411 = t184 * t184;
            let t412 = f64x8::splat(1.0) / t411;
            let t413 = t175 * t412;
            let t415 = f64x8::splat(1.0) / t176 * t7;
            let t416 = t9 * t12;
            let t417 = t416 * t252;
            let t418 = t415 * t417;
            let t420 = t10 * t264;
            let t422 = ((t173).sqrt());
            let t423 = t422 * t7;
            let t424 = t423 * t417;
            let t427 = t32 * t11 * t278;
            let t429 = -f64x8::splat(0.632975) * t418 - f64x8::splat(0.29896666666666666) * t420 - f64x8::splat(0.1023875) * t424 - f64x8::splat(0.08215666666666667) * t427;
            let t430 = f64x8::splat(1.0) / t187;
            let t431 = t429 * t430;
            let t434 = t194 * t7;
            let t439 = t194 * t196;
            let t440 = t201 * t201;
            let t441 = f64x8::splat(1.0) / t440;
            let t446 = -f64x8::splat(0.8630833333333333) * t418 - f64x8::splat(0.301925) * t420 - f64x8::splat(0.05501625) * t424 - f64x8::splat(0.082785) * t427;
            let t448 = f64x8::splat(1.0) / t204;
            let t449 = t441 * t446 * t448;
            let t453 = f64x8::splat(0.0011073470983333333) * t10 * t264 * t188 + f64x8::splat(1.0) * t413 * t431 - f64x8::splat(0.00018311447306006544) * t434 * t416 * t252 * t205 - f64x8::splat(0.5848223622634646) * t439 * t449 - f64x8::splat(2.0) * t335;
            let t454 = t453 * t249;
            let t455 = t341 * t219;
            let t458 = t212 * t224;
            let t459 = t458 * t111;
            let t460 = t350 * t228;
            let t465 = t223 * t233;
            let t466 = t359 * t236;
            let t471 = t232 * t241;
            let t472 = t471 * t138;
            let t474 = t369 * t245 * t36;
            let t479 = t241 * param_gamma_ab;
            let t480 = t240 * t479;
            let t481 = t480 * t378;
            let t483 = f64x8::splat(1.0) / t244 / t218;
            let t484 = t383 * t483;
            let t487 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t214 * t455 + f64x8::splat(64.0) / f64x8::splat(3.0) * t459 * t460 - f64x8::splat(128.0) / f64x8::splat(3.0) * t226 * t460 + f64x8::splat(512.0) / f64x8::splat(3.0) * t465 * t466 - f64x8::splat(256.0) * t234 * t466 + f64x8::splat(512.0) * t472 * t474 - f64x8::splat(2048.0) / f64x8::splat(3.0) * t243 * t474 + f64x8::splat(8192.0) / f64x8::splat(3.0) * t481 * t484;
            let t488 = t210 * t487;
            let tvrho0 = t171 + t250 + v_rho * (f64x8::splat(2.0) * t337 + f64x8::splat(2.0) * t391 + t397 / f64x8::splat(4.0) - f64x8::splat(160.0) / f64x8::splat(3.0) * t406 + t454 + t488);
            acc_vrho = tvrho0;
            let t495 = t110 * v_sigma;
            let t498 = t111 * t128;
            let t499 = t498 * t131;
            let t504 = t365 * t126;
            let t507 = t137 * t126;
            let t510 = t377 * t138;
            let t511 = t127 * t114;
            let t513 = f64x8::splat(1.0) / t14 / t511;
            let t514 = t16 * t513;
            let t515 = t514 * t385;
            let t518 = -f64x8::splat(2.0) * t345 * v_sigma * t120 + t96 * t106 + f64x8::splat(4.0) * t495 * t120 + f64x8::splat(12.0) * t125 * t499 - f64x8::splat(12.0) * t504 * t146 + f64x8::splat(16.0) * t507 * t146 - f64x8::splat(8.0) * t356 * t499 - f64x8::splat(32.0) * t510 * t515;
            let t519 = t93 * t518;
            let t521 = f64x8::splat(2.0) * t519 * t169;
            let t523 = t151 * t153 * t168;
            let t525 = t150 * t523 / f64x8::splat(4.0);
            let t531 = t225 * v_sigma;
            let t534 = t498 * t236;
            let t539 = t471 * t126;
            let t542 = t242 * t126;
            let t545 = t480 * t138;
            let t546 = t514 * t483;
            let t549 = -f64x8::splat(8.0) * t458 * v_sigma * t229 + f64x8::splat(2.0) * t213 * t220 + f64x8::splat(16.0) * t531 * t229 + f64x8::splat(96.0) * t234 * t534 - f64x8::splat(192.0) * t539 * t246 + f64x8::splat(256.0) * t542 * t246 - f64x8::splat(64.0) * t465 * t534 - f64x8::splat(1024.0) * t545 * t546;
            let t550 = t210 * t549;
            let tvsigma0 = v_rho * (t521 - t525 + t550);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t552 = f64x8::splat(1.0) / t157;
            let t554 = t151 * t552 * t168;
            let t556 = t393 * t554 / f64x8::splat(4.0);
            let t557 = t156 * v_tau;
            let t558 = t150 * t557;
            let t559 = t16 * t161;
            let t560 = t559 * t404;
            let t562 = f64x8::splat(32.0) * t558 * t560;
            let tvtau0 = v_rho * (t556 + t562);
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
