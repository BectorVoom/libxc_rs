//! MGGA_C_M05 fxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_c_m05_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
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
            let t571 = f64x8::splat(1.0) / t14 / t98;
            let t572 = t571 * t16;
            let t575 = f64x8::splat(0.0014764627977777779) * t13 * t572 * t254;
            let t578 = t265 * t259 * t283 * t284;
            let t580 = f64x8::splat(0.035616666666666665) * t420 * t578;
            let t581 = t258 * t42;
            let t582 = f64x8::splat(1.0) / t581;
            let t583 = t24 * t582;
            let t584 = t283 * t283;
            let t585 = t584 * t284;
            let t587 = f64x8::splat(2.0) * t583 * t585;
            let t591 = f64x8::splat(1.0) / t25 / t22 * t30 * t31;
            let t592 = t11 * t100;
            let t593 = t36 * t38;
            let t594 = t592 * t593;
            let t595 = t591 * t594;
            let t597 = t12 * t571;
            let t598 = t597 * t265;
            let t599 = t263 * t598;
            let t601 = t572 * t20;
            let t602 = t13 * t601;
            let t604 = f64x8::splat(1.0)/((t22).sqrt());
            let t606 = t604 * t30 * t31;
            let t607 = t606 * t594;
            let t609 = t274 * t598;
            let t612 = t33 * t101 * t38;
            let t614 = -f64x8::splat(0.4219833333333333) * t595 + f64x8::splat(0.8439666666666666) * t599 + f64x8::splat(0.3986222222222222) * t602 + f64x8::splat(0.06825833333333334) * t607 + f64x8::splat(0.13651666666666668) * t609 + f64x8::splat(0.1369277777777778) * t612;
            let t617 = f64x8::splat(1.0) * t260 * t614 * t284;
            let t618 = t258 * t258;
            let t619 = f64x8::splat(1.0) / t618;
            let t620 = t24 * t619;
            let t621 = t45 * t45;
            let t622 = f64x8::splat(1.0) / t621;
            let t623 = t584 * t622;
            let t625 = f64x8::splat(16.081979498692537) * t620 * t623;
            let t631 = t265 * t293 * t299 * t300;
            let t634 = t292 * t66;
            let t635 = f64x8::splat(1.0) / t634;
            let t636 = t61 * t635;
            let t637 = t299 * t299;
            let t638 = t637 * t300;
            let t647 = -f64x8::splat(0.7843833333333333) * t595 + f64x8::splat(1.5687666666666666) * t599 + f64x8::splat(0.6886333333333333) * t602 + f64x8::splat(0.14025833333333335) * t607 + f64x8::splat(0.2805166666666667) * t609 + f64x8::splat(0.17365833333333333) * t612;
            let t651 = t292 * t292;
            let t652 = f64x8::splat(1.0) / t651;
            let t653 = t61 * t652;
            let t654 = t69 * t69;
            let t655 = f64x8::splat(1.0) / t654;
            let t656 = t637 * t655;
            let t662 = t265 * t329;
            let t665 = t308 * t79;
            let t666 = f64x8::splat(1.0) / t665;
            let t667 = t74 * t666;
            let t668 = t315 * t315;
            let t669 = t668 * t316;
            let t678 = -f64x8::splat(0.5753888888888888) * t595 + f64x8::splat(1.1507777777777777) * t599 + f64x8::splat(0.4025666666666667) * t602 + f64x8::splat(0.0366775) * t607 + f64x8::splat(0.073355) * t609 + f64x8::splat(0.137975) * t612;
            let t679 = t678 * t316;
            let t682 = t308 * t308;
            let t683 = f64x8::splat(1.0) / t682;
            let t684 = t74 * t683;
            let t685 = t82 * t82;
            let t686 = f64x8::splat(1.0) / t685;
            let t687 = t668 * t686;
            let t690 = -f64x8::splat(0.0007098352262222222) * t13 * t572 * t288 - f64x8::splat(0.03424666666666667) * t420 * t631 - f64x8::splat(2.0) * t636 * t638 + f64x8::splat(1.0) * t294 * t647 * t300 + f64x8::splat(32.16395899738507) * t653 * t656 + t575 + t580 + t587 - t617 - t625 - f64x8::splat(0.00024415263074675396) * t13 * t572 * t304 - f64x8::splat(0.01084358130030174) * t420 * t662 - f64x8::splat(1.1696447245269292) * t667 * t669 + f64x8::splat(0.5848223622634646) * t310 * t679 + f64x8::splat(17.315859105681465) * t684 * t687;
            let t695 = t59 * t13;
            let t700 = t666 * t668 * t316;
            let t704 = t309 * t678 * t316;
            let t707 = t683 * t668;
            let t708 = t707 * t686;
            let t711 = -t575 - t580 - t587 + t617 + t625 + t59 * t690 + f64x8::splat(0.00024415263074675396) * t322 * t597 * t323 + f64x8::splat(0.01084358130030174) * t695 * t269 * t329 + f64x8::splat(1.1696447245269292) * t327 * t700 - f64x8::splat(0.5848223622634646) * t327 * t704 - f64x8::splat(17.315859105681465) * t327 * t708;
            let t714 = ((t5).select(f64x8::splat(0.0), t6 * t711 / f64x8::splat(2.0)));
            let t715 = t714 * t149;
            let t716 = t715 * t169;
            let t718 = t335 * t389;
            let t719 = t718 * t169;
            let t721 = t336 * v_sigma;
            let t722 = t721 * t396;
            let t724 = t336 * t399;
            let t725 = t724 * t405;
            let t728 = f64x8::splat(1.0) / t34 / t113;
            let t729 = t36 * t728;
            let t730 = t729 * t105;
            let t733 = t113 * t159;
            let t735 = f64x8::splat(1.0) / t14 / t733;
            let t736 = t16 * t735;
            let t737 = t736 * t119;
            let t740 = t95 * t124;
            let t741 = f64x8::splat(1.0) / t140;
            let t742 = t126 * t741;
            let t743 = t742 * t131;
            let t750 = t108 * t136;
            let t751 = t750 * t138;
            let t752 = t127 * t113;
            let t754 = f64x8::splat(1.0) / t34 / t752;
            let t756 = t754 * t145 * t36;
            let t763 = t123 * t376;
            let t764 = t763 * t378;
            let t765 = t127 * t733;
            let t767 = f64x8::splat(1.0) / t14 / t765;
            let t769 = t767 * t385 * t16;
            let t776 = t136 * t109;
            let t777 = t135 * t776;
            let t778 = t138 * t111;
            let t779 = t127 * t127;
            let t780 = t779 * t98;
            let t781 = f64x8::splat(1.0) / t780;
            let t782 = t778 * t781;
            let t784 = f64x8::splat(1.0) / t144 / t118;
            let t788 = f64x8::splat(88.0) / f64x8::splat(9.0) * t97 * t730 - f64x8::splat(48.0) * t346 * t737 + f64x8::splat(512.0) / f64x8::splat(9.0) * t740 * t743 + f64x8::splat(608.0) / f64x8::splat(9.0) * t112 * t737 - f64x8::splat(2752.0) / f64x8::splat(9.0) * t356 * t743 + f64x8::splat(512.0) / f64x8::splat(3.0) * t751 * t756 + f64x8::splat(288.0) * t125 * t743 - f64x8::splat(1888.0) / f64x8::splat(3.0) * t366 * t756 + f64x8::splat(2048.0) / f64x8::splat(3.0) * t764 * t769 + f64x8::splat(4480.0) / f64x8::splat(9.0) * t139 * t756 - f64x8::splat(6400.0) / f64x8::splat(3.0) * t379 * t769 + f64x8::splat(20480.0) / f64x8::splat(9.0) * t777 * t782 * t784;
            let t789 = t93 * t788;
            let t790 = t789 * t169;
            let t792 = t390 * v_sigma;
            let t793 = t792 * t396;
            let t795 = t390 * t399;
            let t796 = t795 * t405;
            let t798 = f64x8::splat(1.0) / t159;
            let t800 = t798 * t153 * t168;
            let t801 = t393 * t800;
            let t803 = v_sigma * t349;
            let t804 = t150 * t803;
            let t806 = v_tau * t16 * t404;
            let t807 = t804 * t806;
            let t809 = t117 * t404;
            let t810 = t400 * t809;
            let t812 = t157 * t157;
            let t813 = t156 * t812;
            let t814 = t150 * t813;
            let t816 = f64x8::splat(1.0) / t34 / t127;
            let t818 = t162 * t162;
            let t819 = f64x8::splat(1.0) / t818;
            let t820 = t819 * t167;
            let t821 = t36 * t816 * t820;
            let t822 = t814 * t821;
            let t827 = t252 * t412;
            let t831 = t411 * t184;
            let t832 = f64x8::splat(1.0) / t831;
            let t833 = t175 * t832;
            let t834 = t429 * t429;
            let t835 = t834 * t430;
            let t840 = f64x8::splat(1.0) / t176 / t173 * t30;
            let t841 = t31 * t11;
            let t842 = t841 * t100;
            let t843 = t840 * t842;
            let t845 = t416 * t571;
            let t846 = t415 * t845;
            let t848 = t10 * t597;
            let t850 = f64x8::splat(1.0)/((t173).sqrt());
            let t851 = t850 * t30;
            let t852 = t851 * t842;
            let t854 = t423 * t845;
            let t856 = t32 * t592;
            let t858 = -f64x8::splat(0.4219833333333333) * t843 + f64x8::splat(0.8439666666666666) * t846 + f64x8::splat(0.3986222222222222) * t848 + f64x8::splat(0.06825833333333334) * t852 + f64x8::splat(0.13651666666666668) * t854 + f64x8::splat(0.1369277777777778) * t856;
            let t859 = t858 * t430;
            let t862 = t411 * t411;
            let t863 = f64x8::splat(1.0) / t862;
            let t864 = t175 * t863;
            let t865 = t187 * t187;
            let t866 = f64x8::splat(1.0) / t865;
            let t867 = t834 * t866;
            let t874 = t194 * t10;
            let t878 = t440 * t201;
            let t879 = f64x8::splat(1.0) / t878;
            let t880 = t446 * t446;
            let t882 = t879 * t880 * t448;
            let t891 = -f64x8::splat(0.5753888888888888) * t843 + f64x8::splat(1.1507777777777777) * t846 + f64x8::splat(0.4025666666666667) * t848 + f64x8::splat(0.0366775) * t852 + f64x8::splat(0.073355) * t854 + f64x8::splat(0.137975) * t856;
            let t893 = t441 * t891 * t448;
            let t896 = t440 * t440;
            let t897 = f64x8::splat(1.0) / t896;
            let t898 = t897 * t880;
            let t899 = t204 * t204;
            let t900 = f64x8::splat(1.0) / t899;
            let t901 = t898 * t900;
            let t905 = -f64x8::splat(0.0014764627977777779) * t10 * t597 * t188 - f64x8::splat(0.035616666666666665) * t13 * t827 * t431 - f64x8::splat(2.0) * t833 * t835 + f64x8::splat(1.0) * t413 * t859 + f64x8::splat(16.081979498692537) * t864 * t867 + f64x8::splat(0.00024415263074675396) * t434 * t416 * t571 * t205 + f64x8::splat(0.01084358130030174) * t874 * t264 * t449 + f64x8::splat(1.1696447245269292) * t439 * t882 - f64x8::splat(0.5848223622634646) * t439 * t893 - f64x8::splat(17.315859105681465) * t439 * t901 - f64x8::splat(2.0) * t714;
            let t906 = t905 * t249;
            let t907 = t453 * t487;
            let t909 = t729 * t219;
            let t912 = t736 * t228;
            let t915 = t212 * t233;
            let t916 = t742 * t236;
            let t923 = t223 * t241;
            let t924 = t923 * t138;
            let t926 = t754 * t245 * t36;
            let t933 = t232 * t479;
            let t934 = t933 * t378;
            let t936 = t767 * t483 * t16;
            let t943 = t241 * t224;
            let t944 = t240 * t943;
            let t946 = f64x8::splat(1.0) / t244 / t227;
            let t950 = f64x8::splat(176.0) / f64x8::splat(9.0) * t214 * t909 - f64x8::splat(192.0) * t459 * t912 + f64x8::splat(4096.0) / f64x8::splat(9.0) * t915 * t916 + f64x8::splat(2432.0) / f64x8::splat(9.0) * t226 * t912 - f64x8::splat(22016.0) / f64x8::splat(9.0) * t465 * t916 + f64x8::splat(8192.0) / f64x8::splat(3.0) * t924 * t926 + f64x8::splat(2304.0) * t234 * t916 - f64x8::splat(30208.0) / f64x8::splat(3.0) * t472 * t926 + f64x8::splat(65536.0) / f64x8::splat(3.0) * t934 * t936 + f64x8::splat(71680.0) / f64x8::splat(9.0) * t243 * t926 - f64x8::splat(204800.0) / f64x8::splat(3.0) * t481 * t936 + f64x8::splat(1310720.0) / f64x8::splat(9.0) * t944 * t782 * t946;
            let t951 = t210 * t950;
            let t952 = f64x8::splat(2.0) * t716 + f64x8::splat(4.0) * t719 + t722 / f64x8::splat(2.0) - f64x8::splat(320.0) / f64x8::splat(3.0) * t725 + f64x8::splat(2.0) * t790 + t793 / f64x8::splat(2.0) - f64x8::splat(320.0) / f64x8::splat(3.0) * t796 - t801 / f64x8::splat(2.0) - f64x8::splat(40.0) / f64x8::splat(3.0) * t807 + f64x8::splat(2080.0) / f64x8::splat(9.0) * t810 - f64x8::splat(12800.0) / f64x8::splat(9.0) * t822 + t906 + f64x8::splat(2.0) * t907 + t951;
            let tv2rho20 = f64x8::splat(4.0) * t337 + f64x8::splat(4.0) * t391 + t397 / f64x8::splat(2.0) - f64x8::splat(320.0) / f64x8::splat(3.0) * t406 + f64x8::splat(2.0) * t454 + f64x8::splat(2.0) * t488 + v_rho * t952;
            acc_v2rho2 = tv2rho20;
            let t954 = t335 * t518;
            let t955 = t954 * t169;
            let t959 = t345 * t16;
            let t964 = t111 * t358;
            let t965 = t964 * t131;
            let t972 = t750 * t126;
            let t979 = t763 * t138;
            let t986 = t779 * v_rho;
            let t987 = f64x8::splat(1.0) / t986;
            let t988 = t378 * t987;
            let t992 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t96 * t342 + f64x8::splat(16.0) * t959 * t349 * t119 * v_sigma - f64x8::splat(64.0) / f64x8::splat(3.0) * t740 * t965 - f64x8::splat(64.0) / f64x8::splat(3.0) * t495 * t351 + f64x8::splat(320.0) / f64x8::splat(3.0) * t356 * t965 - f64x8::splat(64.0) * t972 * t371 - f64x8::splat(96.0) * t125 * t965 + f64x8::splat(224.0) * t504 * t371 - f64x8::splat(256.0) * t979 * t386 - f64x8::splat(512.0) / f64x8::splat(3.0) * t507 * t371 + f64x8::splat(768.0) * t510 * t386 - f64x8::splat(2560.0) / f64x8::splat(3.0) * t777 * t988 * t784;
            let t993 = t93 * t992;
            let t994 = t993 * t169;
            let t996 = t519 * v_sigma;
            let t997 = t996 * t396;
            let t999 = t519 * t399;
            let t1000 = t999 * t405;
            let t1002 = t336 * t523;
            let t1004 = t390 * t523;
            let t1006 = t150 * t396;
            let t1008 = t150 * t116;
            let t1009 = t1008 * t806;
            let t1011 = t453 * t549;
            let t1014 = t458 * t16;
            let t1019 = t964 * t236;
            let t1026 = t923 * t126;
            let t1033 = t933 * t138;
            let t1043 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t213 * t455 + f64x8::splat(64.0) * t1014 * t349 * t228 * v_sigma - f64x8::splat(512.0) / f64x8::splat(3.0) * t915 * t1019 - f64x8::splat(256.0) / f64x8::splat(3.0) * t531 * t460 + f64x8::splat(2560.0) / f64x8::splat(3.0) * t465 * t1019 - f64x8::splat(1024.0) * t1026 * t474 - f64x8::splat(768.0) * t234 * t1019 + f64x8::splat(3584.0) * t539 * t474 - f64x8::splat(8192.0) * t1033 * t484 - f64x8::splat(8192.0) / f64x8::splat(3.0) * t542 * t474 + f64x8::splat(24576.0) * t545 * t484 - f64x8::splat(163840.0) / f64x8::splat(3.0) * t944 * t988 * t946;
            let t1044 = t210 * t1043;
            let tv2rhosigma0 = t521 - t525 + t550 + v_rho * (f64x8::splat(2.0) * t955 + f64x8::splat(2.0) * t994 + t997 / f64x8::splat(4.0) - f64x8::splat(160.0) / f64x8::splat(3.0) * t1000 - t1002 / f64x8::splat(4.0) - t1004 / f64x8::splat(4.0) + t1006 / f64x8::splat(4.0) + f64x8::splat(20.0) / f64x8::splat(3.0) * t1009 + t1011 + t1044);
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t1047 = t721 * t554;
            let t1049 = t792 * t554;
            let t1052 = t394 * t552 * t168;
            let t1053 = t393 * t1052;
            let t1055 = t393 * t809;
            let t1057 = t336 * t557;
            let t1058 = t1057 * t560;
            let t1060 = t390 * t557;
            let t1061 = t1060 * t560;
            let t1063 = t558 * t405;
            let t1065 = t157 * v_tau;
            let t1066 = t156 * t1065;
            let t1067 = t150 * t1066;
            let t1069 = f64x8::splat(1.0) / t34 / t733;
            let t1071 = t36 * t1069 * t820;
            let t1072 = t1067 * t1071;
            let tv2rhotau0 = t556 + t562 + v_rho * (t1047 / f64x8::splat(4.0) + t1049 / f64x8::splat(4.0) - t1053 / f64x8::splat(4.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t1055 + f64x8::splat(32.0) * t1058 + f64x8::splat(32.0) * t1061 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1063 + f64x8::splat(2560.0) / f64x8::splat(3.0) * t1072);
            acc_v2rhotau = tv2rhotau0;
            let t1078 = v_sigma * t128;
            let t1079 = t1078 * t131;
            let t1086 = t750 * t111;
            let t1091 = t365 * t111;
            let t1094 = t763 * t126;
            let t1097 = t137 * t111;
            let t1100 = t377 * t126;
            let t1103 = f64x8::splat(1.0) / t779;
            let t1104 = t138 * t1103;
            let t1108 = f64x8::splat(320.0) * t777 * t1104 * t784 + f64x8::splat(24.0) * t125 * t1079 - f64x8::splat(32.0) * t356 * t1079 + f64x8::splat(8.0) * t740 * t1079 + f64x8::splat(24.0) * t1086 * t146 - f64x8::splat(72.0) * t1091 * t146 + f64x8::splat(96.0) * t1094 * t515 + f64x8::splat(48.0) * t1097 * t146 + f64x8::splat(4.0) * t110 * t120 - f64x8::splat(256.0) * t1100 * t515 - f64x8::splat(4.0) * t345 * t120;
            let t1109 = t93 * t1108;
            let t1111 = f64x8::splat(2.0) * t1109 * t169;
            let t1113 = t519 * t523 / f64x8::splat(2.0);
            let t1116 = t1078 * t236;
            let t1123 = t923 * t111;
            let t1128 = t471 * t111;
            let t1131 = t933 * t126;
            let t1134 = t242 * t111;
            let t1137 = t480 * t126;
            let t1143 = f64x8::splat(20480.0) * t944 * t1104 * t946 + f64x8::splat(192.0) * t234 * t1116 - f64x8::splat(256.0) * t465 * t1116 + f64x8::splat(64.0) * t915 * t1116 + f64x8::splat(384.0) * t1123 * t246 - f64x8::splat(1152.0) * t1128 * t246 + f64x8::splat(3072.0) * t1131 * t546 + f64x8::splat(768.0) * t1134 * t246 - f64x8::splat(8192.0) * t1137 * t546 + f64x8::splat(16.0) * t225 * t229 - f64x8::splat(16.0) * t458 * t229;
            let t1144 = t210 * t1143;
            let tv2sigma20 = v_rho * (t1111 - t1113 + t1144);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t1147 = t996 * t554 / f64x8::splat(4.0);
            let t1149 = t150 * t554 / f64x8::splat(4.0);
            let t1150 = t519 * t557;
            let t1152 = f64x8::splat(32.0) * t1150 * t560;
            let t1153 = t150 * t402;
            let t1155 = t16 * t163 * t167;
            let t1157 = f64x8::splat(4.0) * t1153 * t1155;
            let tv2sigmatau0 = v_rho * (t1147 + t1149 + t1152 - t1157);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1159 = f64x8::splat(1.0) / t1065;
            let t1161 = t151 * t1159 * t168;
            let t1163 = t393 * t1161 / f64x8::splat(2.0);
            let t1164 = v_sigma * t402;
            let t1165 = t150 * t1164;
            let t1167 = t153 * t16 * t404;
            let t1169 = f64x8::splat(8.0) * t1165 * t1167;
            let t1170 = t150 * t156;
            let t1172 = f64x8::splat(32.0) * t1170 * t560;
            let t1174 = f64x8::splat(1.0) / t34 / t347;
            let t1175 = t36 * t1174;
            let t1176 = t1175 * t820;
            let t1178 = f64x8::splat(512.0) * t400 * t1176;
            let tv2tau20 = v_rho * (-t1163 + t1169 + t1172 - t1178);
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
