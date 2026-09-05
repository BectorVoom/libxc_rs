//! MGGA_C_BC95 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`
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
pub fn mgga_c_bc95_fxc_unpol(
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
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_copp = f64x8::splat(param_copp);
    let param_css = f64x8::splat(param_css);
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
            let t94 = t93 * v_tau;
            let t96 = f64x8::splat(1.0) / t34 / v_rho;
            let t97 = t36 * t96;
            let t99 = f64x8::splat(1.0) / v_rho;
            let t101 = f64x8::splat(1.0) / v_tau;
            let t104 = f64x8::splat(1.0) - v_sigma * t99 * t101 / f64x8::splat(8.0);
            let t105 = f64x8::splat(M_CBRT6);
            let t106 = t104 * t105;
            let t107 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t108 = (simd::cbrt(t107));
            let t109 = t108 * t108;
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = param_css * v_sigma;
            let t112 = v_rho * v_rho;
            let t114 = f64x8::splat(1.0) / t34 / t112;
            let t115 = t36 * t114;
            let t117 = t111 * t115 + f64x8::splat(1.0);
            let t118 = t117 * t117;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t110 * t119;
            let t121 = t106 * t120;
            let t123 = f64x8::splat(10.0) / f64x8::splat(9.0) * t94 * t97 * t121;
            let t125 = t10 * t12 * t15;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t125;
            let t128 = ((t125).sqrt());
            let t131 = ((t125) * (t125).sqrt());
            let t134 = t32 * t11 * t35;
            let t136 = f64x8::splat(3.79785) * t128 + f64x8::splat(0.8969) * t125 + f64x8::splat(0.204775) * t131 + f64x8::splat(0.123235) * t134;
            let t139 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t136;
            let t140 = (simd::ln(t139));
            let t143 = ((t4).select(t50, f64x8::splat(1.0)));
            let t146 = (f64x8::splat(2.0) * t143 - f64x8::splat(2.0)) * t58;
            let t148 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t125;
            let t153 = f64x8::splat(5.1785) * t128 + f64x8::splat(0.905775) * t125 + f64x8::splat(0.1100325) * t131 + f64x8::splat(0.1241775) * t134;
            let t156 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t153;
            let t157 = (simd::ln(t156));
            let t162 = -f64x8::splat(0.0621814) * t127 * t140 + f64x8::splat(0.0197516734986138) * t146 * t148 * t157 - f64x8::splat(2.0) * t93;
            let t166 = f64x8::splat(2.0) * param_copp * v_sigma * t115 + f64x8::splat(1.0);
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t162 * t167;
            let tzk0 = t123 + t168;
            acc_zk = tzk0;
            let t170 = f64x8::splat(1.0) / t14 / v_rho;
            let t171 = t170 * t16;
            let t172 = t20 * t46;
            let t175 = f64x8::splat(0.0011073470983333333) * t13 * t171 * t172;
            let t176 = t42 * t42;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t24 * t177;
            let t181 = f64x8::splat(1.0) / t25 * t7 * t9;
            let t182 = t12 * t170;
            let t183 = t16 * t20;
            let t184 = t182 * t183;
            let t185 = t181 * t184;
            let t187 = t171 * t20;
            let t188 = t13 * t187;
            let t190 = ((t22).sqrt());
            let t192 = t190 * t7 * t9;
            let t193 = t192 * t184;
            let t196 = t33 * t97 * t38;
            let t198 = -f64x8::splat(0.632975) * t185 - f64x8::splat(0.29896666666666666) * t188 - f64x8::splat(0.1023875) * t193 - f64x8::splat(0.08215666666666667) * t196;
            let t199 = f64x8::splat(1.0) / t45;
            let t200 = t198 * t199;
            let t202 = f64x8::splat(1.0) * t178 * t200;
            let t203 = t20 * t70;
            let t207 = t66 * t66;
            let t208 = f64x8::splat(1.0) / t207;
            let t209 = t61 * t208;
            let t214 = -f64x8::splat(1.176575) * t185 - f64x8::splat(0.516475) * t188 - f64x8::splat(0.2103875) * t193 - f64x8::splat(0.104195) * t196;
            let t215 = f64x8::splat(1.0) / t69;
            let t216 = t214 * t215;
            let t219 = t20 * t83;
            let t223 = t79 * t79;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t74 * t224;
            let t230 = -f64x8::splat(0.8630833333333333) * t185 - f64x8::splat(0.301925) * t188 - f64x8::splat(0.05501625) * t193 - f64x8::splat(0.082785) * t196;
            let t231 = f64x8::splat(1.0) / t82;
            let t232 = t230 * t231;
            let t237 = t59 * t10;
            let t238 = t183 * t83;
            let t242 = t59 * t74;
            let t244 = t224 * t230 * t231;
            let t250 = ((t5).select(f64x8::splat(0.0), t6 * (t175 + t202 + t59 * (f64x8::splat(0.0005323764196666666) * t13 * t171 * t203 + f64x8::splat(1.0) * t209 * t216 - t175 - t202 + f64x8::splat(0.00018311447306006544) * t13 * t171 * t219 + f64x8::splat(0.5848223622634646) * t225 * t232) - f64x8::splat(0.00018311447306006544) * t237 * t182 * t238 - f64x8::splat(0.5848223622634646) * t242 * t244) / f64x8::splat(2.0)));
            let t251 = t250 * v_tau;
            let t253 = t251 * t97 * t121;
            let t256 = t94 * t115 * t121;
            let t258 = t93 * t36;
            let t259 = t112 * v_rho;
            let t261 = f64x8::splat(1.0) / t34 / t259;
            let t262 = t258 * t261;
            let t264 = v_sigma * t105 * t120;
            let t265 = t262 * t264;
            let t267 = t112 * t112;
            let t268 = t267 * v_rho;
            let t270 = f64x8::splat(1.0) / t14 / t268;
            let t271 = t16 * t270;
            let t272 = t271 * t104;
            let t274 = t105 * t110;
            let t276 = f64x8::splat(1.0) / t118 / t117;
            let t277 = t276 * param_css;
            let t279 = t274 * t277 * v_sigma;
            let t280 = t94 * t272 * t279;
            let t285 = t136 * t136;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t127 * t286;
            let t289 = f64x8::splat(1.0) / t128 * t7;
            let t290 = t9 * t12;
            let t291 = t290 * t170;
            let t292 = t289 * t291;
            let t294 = t10 * t182;
            let t296 = ((t125).sqrt());
            let t297 = t296 * t7;
            let t298 = t297 * t291;
            let t301 = t32 * t11 * t96;
            let t303 = -f64x8::splat(0.632975) * t292 - f64x8::splat(0.29896666666666666) * t294 - f64x8::splat(0.1023875) * t298 - f64x8::splat(0.08215666666666667) * t301;
            let t304 = f64x8::splat(1.0) / t139;
            let t305 = t303 * t304;
            let t308 = t146 * t7;
            let t313 = t146 * t148;
            let t314 = t153 * t153;
            let t315 = f64x8::splat(1.0) / t314;
            let t320 = -f64x8::splat(0.8630833333333333) * t292 - f64x8::splat(0.301925) * t294 - f64x8::splat(0.05501625) * t298 - f64x8::splat(0.082785) * t301;
            let t322 = f64x8::splat(1.0) / t156;
            let t323 = t315 * t320 * t322;
            let t327 = f64x8::splat(0.0011073470983333333) * t10 * t182 * t140 + f64x8::splat(1.0) * t287 * t305 - f64x8::splat(0.00018311447306006544) * t308 * t290 * t170 * t157 - f64x8::splat(0.5848223622634646) * t313 * t323 - f64x8::splat(2.0) * t250;
            let t328 = t327 * t167;
            let t329 = t166 * t166;
            let t330 = f64x8::splat(1.0) / t329;
            let t331 = t162 * t330;
            let t332 = t331 * param_copp;
            let t333 = v_sigma * t36;
            let t334 = t333 * t261;
            let t335 = t332 * t334;
            let tvrho0 = t123 + t168 + v_rho * (f64x8::splat(10.0) / f64x8::splat(9.0) * t253 - f64x8::splat(50.0) / f64x8::splat(27.0) * t256 + f64x8::splat(5.0) / f64x8::splat(36.0) * t265 + f64x8::splat(320.0) / f64x8::splat(27.0) * t280 + t328 + f64x8::splat(16.0) / f64x8::splat(3.0) * t335);
            acc_vrho = tvrho0;
            let t339 = t258 * t114;
            let t340 = t274 * t119;
            let t342 = f64x8::splat(5.0) / f64x8::splat(36.0) * t339 * t340;
            let t344 = f64x8::splat(1.0) / t14 / t267;
            let t345 = t16 * t344;
            let t347 = t110 * t276;
            let t348 = t347 * param_css;
            let t349 = t106 * t348;
            let t351 = f64x8::splat(40.0) / f64x8::splat(9.0) * t94 * t345 * t349;
            let t352 = param_copp * t36;
            let t353 = t352 * t114;
            let t355 = f64x8::splat(2.0) * t331 * t353;
            let tvsigma0 = v_rho * (-t342 - t351 - t355);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t359 = f64x8::splat(10.0) / f64x8::splat(9.0) * t258 * t96 * t121;
            let t360 = t93 * t101;
            let t363 = f64x8::splat(5.0) / f64x8::splat(36.0) * t360 * t115 * t264;
            let tvtau0 = v_rho * (t359 + t363);
            acc_vtau = tvtau0;
            let t372 = f64x8::splat(1.0) / t14 / t112;
            let t373 = t372 * t16;
            let t376 = f64x8::splat(0.0014764627977777779) * t13 * t373 * t172;
            let t379 = t183 * t177 * t198 * t199;
            let t381 = f64x8::splat(0.035616666666666665) * t294 * t379;
            let t382 = t176 * t42;
            let t383 = f64x8::splat(1.0) / t382;
            let t384 = t24 * t383;
            let t385 = t198 * t198;
            let t386 = t385 * t199;
            let t388 = f64x8::splat(2.0) * t384 * t386;
            let t392 = f64x8::splat(1.0) / t25 / t22 * t30 * t31;
            let t393 = t11 * t114;
            let t394 = t36 * t38;
            let t395 = t393 * t394;
            let t396 = t392 * t395;
            let t398 = t12 * t372;
            let t399 = t398 * t183;
            let t400 = t181 * t399;
            let t402 = t373 * t20;
            let t403 = t13 * t402;
            let t405 = f64x8::splat(1.0)/((t22).sqrt());
            let t407 = t405 * t30 * t31;
            let t408 = t407 * t395;
            let t410 = t192 * t399;
            let t413 = t33 * t115 * t38;
            let t415 = -f64x8::splat(0.4219833333333333) * t396 + f64x8::splat(0.8439666666666666) * t400 + f64x8::splat(0.3986222222222222) * t403 + f64x8::splat(0.06825833333333334) * t408 + f64x8::splat(0.13651666666666668) * t410 + f64x8::splat(0.1369277777777778) * t413;
            let t418 = f64x8::splat(1.0) * t178 * t415 * t199;
            let t419 = t176 * t176;
            let t420 = f64x8::splat(1.0) / t419;
            let t421 = t24 * t420;
            let t422 = t45 * t45;
            let t423 = f64x8::splat(1.0) / t422;
            let t424 = t385 * t423;
            let t426 = f64x8::splat(16.081979498692537) * t421 * t424;
            let t432 = t183 * t208 * t214 * t215;
            let t435 = t207 * t66;
            let t436 = f64x8::splat(1.0) / t435;
            let t437 = t61 * t436;
            let t438 = t214 * t214;
            let t439 = t438 * t215;
            let t448 = -f64x8::splat(0.7843833333333333) * t396 + f64x8::splat(1.5687666666666666) * t400 + f64x8::splat(0.6886333333333333) * t403 + f64x8::splat(0.14025833333333335) * t408 + f64x8::splat(0.2805166666666667) * t410 + f64x8::splat(0.17365833333333333) * t413;
            let t452 = t207 * t207;
            let t453 = f64x8::splat(1.0) / t452;
            let t454 = t61 * t453;
            let t455 = t69 * t69;
            let t456 = f64x8::splat(1.0) / t455;
            let t457 = t438 * t456;
            let t463 = t183 * t244;
            let t466 = t223 * t79;
            let t467 = f64x8::splat(1.0) / t466;
            let t468 = t74 * t467;
            let t469 = t230 * t230;
            let t470 = t469 * t231;
            let t479 = -f64x8::splat(0.5753888888888888) * t396 + f64x8::splat(1.1507777777777777) * t400 + f64x8::splat(0.4025666666666667) * t403 + f64x8::splat(0.0366775) * t408 + f64x8::splat(0.073355) * t410 + f64x8::splat(0.137975) * t413;
            let t480 = t479 * t231;
            let t483 = t223 * t223;
            let t484 = f64x8::splat(1.0) / t483;
            let t485 = t74 * t484;
            let t486 = t82 * t82;
            let t487 = f64x8::splat(1.0) / t486;
            let t488 = t469 * t487;
            let t491 = -f64x8::splat(0.0007098352262222222) * t13 * t373 * t203 - f64x8::splat(0.03424666666666667) * t294 * t432 - f64x8::splat(2.0) * t437 * t439 + f64x8::splat(1.0) * t209 * t448 * t215 + f64x8::splat(32.16395899738507) * t454 * t457 + t376 + t381 + t388 - t418 - t426 - f64x8::splat(0.00024415263074675396) * t13 * t373 * t219 - f64x8::splat(0.01084358130030174) * t294 * t463 - f64x8::splat(1.1696447245269292) * t468 * t470 + f64x8::splat(0.5848223622634646) * t225 * t480 + f64x8::splat(17.315859105681465) * t485 * t488;
            let t496 = t59 * t13;
            let t501 = t467 * t469 * t231;
            let t505 = t224 * t479 * t231;
            let t508 = t484 * t469;
            let t509 = t508 * t487;
            let t512 = -t376 - t381 - t388 + t418 + t426 + t59 * t491 + f64x8::splat(0.00024415263074675396) * t237 * t398 * t238 + f64x8::splat(0.01084358130030174) * t496 * t187 * t244 + f64x8::splat(1.1696447245269292) * t242 * t501 - f64x8::splat(0.5848223622634646) * t242 * t505 - f64x8::splat(17.315859105681465) * t242 * t509;
            let t515 = ((t5).select(f64x8::splat(0.0), t6 * t512 / f64x8::splat(2.0)));
            let t516 = t515 * v_tau;
            let t518 = t516 * t97 * t121;
            let t521 = t251 * t115 * t121;
            let t523 = t250 * t36;
            let t524 = t523 * t261;
            let t525 = t524 * t264;
            let t528 = t251 * t272 * t279;
            let t530 = t36 * t261;
            let t532 = t94 * t530 * t121;
            let t535 = f64x8::splat(1.0) / t34 / t267;
            let t536 = t258 * t535;
            let t537 = t536 * t264;
            let t539 = t267 * t112;
            let t541 = f64x8::splat(1.0) / t14 / t539;
            let t542 = t16 * t541;
            let t543 = t542 * t104;
            let t545 = t94 * t543 * t279;
            let t547 = t93 * t16;
            let t548 = t267 * t259;
            let t550 = f64x8::splat(1.0) / t14 / t548;
            let t551 = v_sigma * v_sigma;
            let t552 = t550 * t551;
            let t553 = t547 * t552;
            let t554 = t274 * t277;
            let t555 = t553 * t554;
            let t557 = t267 * t267;
            let t558 = t557 * v_rho;
            let t559 = f64x8::splat(1.0) / t558;
            let t560 = t559 * t104;
            let t561 = t94 * t560;
            let t562 = t118 * t118;
            let t563 = f64x8::splat(1.0) / t562;
            let t564 = param_css * param_css;
            let t565 = t563 * t564;
            let t567 = t274 * t565 * t551;
            let t568 = t561 * t567;
            let t573 = t170 * t286;
            let t577 = t285 * t136;
            let t578 = f64x8::splat(1.0) / t577;
            let t579 = t127 * t578;
            let t580 = t303 * t303;
            let t581 = t580 * t304;
            let t586 = f64x8::splat(1.0) / t128 / t125 * t30;
            let t587 = t31 * t11;
            let t588 = t587 * t114;
            let t589 = t586 * t588;
            let t591 = t290 * t372;
            let t592 = t289 * t591;
            let t594 = t10 * t398;
            let t596 = f64x8::splat(1.0)/((t125).sqrt());
            let t597 = t596 * t30;
            let t598 = t597 * t588;
            let t600 = t297 * t591;
            let t602 = t32 * t393;
            let t604 = -f64x8::splat(0.4219833333333333) * t589 + f64x8::splat(0.8439666666666666) * t592 + f64x8::splat(0.3986222222222222) * t594 + f64x8::splat(0.06825833333333334) * t598 + f64x8::splat(0.13651666666666668) * t600 + f64x8::splat(0.1369277777777778) * t602;
            let t605 = t604 * t304;
            let t608 = t285 * t285;
            let t609 = f64x8::splat(1.0) / t608;
            let t610 = t127 * t609;
            let t611 = t139 * t139;
            let t612 = f64x8::splat(1.0) / t611;
            let t613 = t580 * t612;
            let t620 = t146 * t10;
            let t624 = t314 * t153;
            let t625 = f64x8::splat(1.0) / t624;
            let t626 = t320 * t320;
            let t628 = t625 * t626 * t322;
            let t637 = -f64x8::splat(0.5753888888888888) * t589 + f64x8::splat(1.1507777777777777) * t592 + f64x8::splat(0.4025666666666667) * t594 + f64x8::splat(0.0366775) * t598 + f64x8::splat(0.073355) * t600 + f64x8::splat(0.137975) * t602;
            let t639 = t315 * t637 * t322;
            let t642 = t314 * t314;
            let t643 = f64x8::splat(1.0) / t642;
            let t644 = t643 * t626;
            let t645 = t156 * t156;
            let t646 = f64x8::splat(1.0) / t645;
            let t647 = t644 * t646;
            let t651 = -f64x8::splat(0.0014764627977777779) * t10 * t398 * t140 - f64x8::splat(0.035616666666666665) * t13 * t573 * t305 - f64x8::splat(2.0) * t579 * t581 + f64x8::splat(1.0) * t287 * t605 + f64x8::splat(16.081979498692537) * t610 * t613 + f64x8::splat(0.00024415263074675396) * t308 * t290 * t372 * t157 + f64x8::splat(0.01084358130030174) * t620 * t182 * t323 + f64x8::splat(1.1696447245269292) * t313 * t628 - f64x8::splat(0.5848223622634646) * t313 * t639 - f64x8::splat(17.315859105681465) * t313 * t647 - f64x8::splat(2.0) * t515;
            let t652 = t651 * t167;
            let t653 = t327 * t330;
            let t654 = t653 * param_copp;
            let t655 = t654 * t334;
            let t658 = f64x8::splat(1.0) / t329 / t166;
            let t659 = t162 * t658;
            let t660 = param_copp * param_copp;
            let t661 = t659 * t660;
            let t662 = t551 * t16;
            let t663 = t662 * t550;
            let t664 = t661 * t663;
            let t666 = t333 * t535;
            let t667 = t332 * t666;
            let t669 = f64x8::splat(10.0) / f64x8::splat(9.0) * t518 - f64x8::splat(100.0) / f64x8::splat(27.0) * t521 + f64x8::splat(5.0) / f64x8::splat(18.0) * t525 + f64x8::splat(640.0) / f64x8::splat(27.0) * t528 + f64x8::splat(400.0) / f64x8::splat(81.0) * t532 - f64x8::splat(20.0) / f64x8::splat(27.0) * t537 - f64x8::splat(2240.0) / f64x8::splat(27.0) * t545 + f64x8::splat(80.0) / f64x8::splat(27.0) * t555 + f64x8::splat(5120.0) / f64x8::splat(27.0) * t568 + t652 + f64x8::splat(32.0) / f64x8::splat(3.0) * t655 + f64x8::splat(1024.0) / f64x8::splat(9.0) * t664 - f64x8::splat(176.0) / f64x8::splat(9.0) * t667;
            let tv2rho20 = f64x8::splat(20.0) / f64x8::splat(9.0) * t253 - f64x8::splat(100.0) / f64x8::splat(27.0) * t256 + f64x8::splat(5.0) / f64x8::splat(18.0) * t265 + f64x8::splat(640.0) / f64x8::splat(27.0) * t280 + f64x8::splat(2.0) * t328 + f64x8::splat(32.0) / f64x8::splat(3.0) * t335 + v_rho * t669;
            acc_v2rho2 = tv2rho20;
            let t671 = t523 * t114;
            let t672 = t671 * t340;
            let t674 = t262 * t340;
            let t676 = t541 * t105;
            let t678 = t347 * t111;
            let t679 = t547 * t676 * t678;
            let t682 = t251 * t345 * t349;
            let t685 = t94 * t271 * t349;
            let t687 = f64x8::splat(1.0) / t557;
            let t688 = t687 * t104;
            let t689 = t94 * t688;
            let t691 = t274 * t565 * v_sigma;
            let t692 = t689 * t691;
            let t694 = t653 * t353;
            let t696 = t542 * v_sigma;
            let t697 = t661 * t696;
            let t699 = t352 * t261;
            let t700 = t331 * t699;
            let tv2rhosigma0 = -t342 - t351 - t355 + v_rho * (-f64x8::splat(5.0) / f64x8::splat(36.0) * t672 + f64x8::splat(10.0) / f64x8::splat(27.0) * t674 - f64x8::splat(55.0) / f64x8::splat(27.0) * t679 - f64x8::splat(40.0) / f64x8::splat(9.0) * t682 + f64x8::splat(520.0) / f64x8::splat(27.0) * t685 - f64x8::splat(640.0) / f64x8::splat(9.0) * t692 - f64x8::splat(2.0) * t694 - f64x8::splat(128.0) / f64x8::splat(3.0) * t697 + f64x8::splat(16.0) / f64x8::splat(3.0) * t700);
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t705 = t523 * t96 * t121;
            let t707 = t339 * t121;
            let t709 = t261 * v_sigma;
            let t711 = t101 * t105;
            let t712 = t711 * t120;
            let t713 = t258 * t709 * t712;
            let t715 = t270 * t104;
            let t716 = t547 * t715;
            let t717 = t716 * t279;
            let t719 = t250 * t101;
            let t721 = t719 * t115 * t264;
            let t725 = t551 * t105 * t348;
            let t726 = t360 * t542 * t725;
            let tv2rhotau0 = t359 + t363 + v_rho * (f64x8::splat(10.0) / f64x8::splat(9.0) * t705 - f64x8::splat(50.0) / f64x8::splat(27.0) * t707 - f64x8::splat(25.0) / f64x8::splat(108.0) * t713 + f64x8::splat(320.0) / f64x8::splat(27.0) * t717 + f64x8::splat(5.0) / f64x8::splat(36.0) * t721 + f64x8::splat(40.0) / f64x8::splat(27.0) * t726);
            acc_v2rhotau = tv2rhotau0;
            let t732 = f64x8::splat(10.0) / f64x8::splat(9.0) * t547 * t270 * t554;
            let t733 = f64x8::splat(1.0) / t548;
            let t734 = t733 * t104;
            let t736 = t274 * t565;
            let t738 = f64x8::splat(80.0) / f64x8::splat(3.0) * t94 * t734 * t736;
            let t739 = t660 * t16;
            let t740 = t739 * t270;
            let t742 = f64x8::splat(16.0) * t659 * t740;
            let tv2sigma20 = v_rho * (t732 + t738 + t742);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t744 = t344 * t104;
            let t747 = f64x8::splat(40.0) / f64x8::splat(9.0) * t547 * t744 * t554;
            let t750 = f64x8::splat(5.0) / f64x8::splat(9.0) * t360 * t271 * t279;
            let tv2sigmatau0 = v_rho * (-t747 - t750);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
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
