//! MGGA_C_VSXC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`
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
pub fn mgga_c_vsxc_fxc_unpol(
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
    param_dss_0: f64,
    param_alpha_ss: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_dab_0: f64,
    param_alpha_ab: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_dss_0 = f64x8::splat(param_dss_0);
    let param_alpha_ss = f64x8::splat(param_alpha_ss);
    let param_dss_1 = f64x8::splat(param_dss_1);
    let param_dss_2 = f64x8::splat(param_dss_2);
    let param_dss_3 = f64x8::splat(param_dss_3);
    let param_dss_4 = f64x8::splat(param_dss_4);
    let param_dss_5 = f64x8::splat(param_dss_5);
    let param_dab_0 = f64x8::splat(param_dab_0);
    let param_alpha_ab = f64x8::splat(param_alpha_ab);
    let param_dab_1 = f64x8::splat(param_dab_1);
    let param_dab_2 = f64x8::splat(param_dab_2);
    let param_dab_3 = f64x8::splat(param_dab_3);
    let param_dab_4 = f64x8::splat(param_dab_4);
    let param_dab_5 = f64x8::splat(param_dab_5);
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
            let t94 = param_dss_0;
            let t95 = v_sigma * t36;
            let t96 = v_rho * v_rho;
            let t98 = f64x8::splat(1.0) / t34 / t96;
            let t99 = t95 * t98;
            let t100 = v_tau * t36;
            let t102 = f64x8::splat(1.0) / t34 / v_rho;
            let t103 = t100 * t102;
            let t104 = f64x8::splat(2.0) * t103;
            let t105 = f64x8::splat(M_CBRT6);
            let t106 = t105 * t105;
            let t107 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t108 = (simd::cbrt(t107));
            let t109 = t108 * t108;
            let t110 = t106 * t109;
            let t111 = f64x8::splat(3.0) / f64x8::splat(5.0) * t110;
            let t114 = f64x8::splat(1.0) + param_alpha_ss * (t99 + t104 - t111);
            let t117 = param_dss_1;
            let t118 = t117 * v_sigma;
            let t119 = t36 * t98;
            let t121 = param_dss_2;
            let t122 = t104 - t111;
            let t124 = t118 * t119 + t121 * t122;
            let t125 = t114 * t114;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = param_dss_3;
            let t129 = v_sigma * v_sigma;
            let t130 = t128 * t129;
            let t131 = t96 * t96;
            let t132 = t131 * v_rho;
            let t134 = f64x8::splat(1.0) / t14 / t132;
            let t135 = t16 * t134;
            let t138 = param_dss_4;
            let t139 = t138 * v_sigma;
            let t142 = param_dss_5;
            let t143 = t122 * t122;
            let t145 = t119 * t122 * t139 + f64x8::splat(2.0) * t130 * t135 + t142 * t143;
            let t146 = t125 * t114;
            let t147 = f64x8::splat(1.0) / t146;
            let t149 = t94 / t114 + t124 * t126 + t145 * t147;
            let t150 = t93 * t149;
            let t151 = f64x8::splat(1.0) / v_rho;
            let t152 = v_sigma * t151;
            let t153 = f64x8::splat(1.0) / v_tau;
            let t156 = f64x8::splat(1.0) - t152 * t153 / f64x8::splat(8.0);
            let t158 = f64x8::splat(2.0) * t150 * t156;
            let t160 = t10 * t12 * t15;
            let t162 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t160;
            let t163 = ((t160).sqrt());
            let t166 = ((t160) * (t160).sqrt());
            let t169 = t32 * t11 * t35;
            let t171 = f64x8::splat(3.79785) * t163 + f64x8::splat(0.8969) * t160 + f64x8::splat(0.204775) * t166 + f64x8::splat(0.123235) * t169;
            let t174 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t171;
            let t175 = (simd::ln(t174));
            let t178 = ((t4).select(t50, f64x8::splat(1.0)));
            let t181 = (f64x8::splat(2.0) * t178 - f64x8::splat(2.0)) * t58;
            let t183 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t160;
            let t188 = f64x8::splat(5.1785) * t163 + f64x8::splat(0.905775) * t160 + f64x8::splat(0.1100325) * t166 + f64x8::splat(0.1241775) * t169;
            let t191 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t188;
            let t192 = (simd::ln(t191));
            let t197 = -f64x8::splat(0.0621814) * t162 * t175 + f64x8::splat(0.0197516734986138) * t181 * t183 * t192 - f64x8::splat(2.0) * t93;
            let t198 = param_dab_0;
            let t200 = f64x8::splat(4.0) * t103;
            let t201 = f64x8::splat(6.0) / f64x8::splat(5.0) * t110;
            let t204 = f64x8::splat(1.0) + param_alpha_ab * (f64x8::splat(2.0) * t99 + t200 - t201);
            let t207 = param_dab_1;
            let t208 = t207 * v_sigma;
            let t211 = param_dab_2;
            let t212 = t200 - t201;
            let t214 = f64x8::splat(2.0) * t119 * t208 + t211 * t212;
            let t215 = t204 * t204;
            let t216 = f64x8::splat(1.0) / t215;
            let t218 = param_dab_3;
            let t219 = t218 * t129;
            let t222 = param_dab_4;
            let t223 = t222 * v_sigma;
            let t227 = param_dab_5;
            let t228 = t212 * t212;
            let t230 = f64x8::splat(2.0) * t119 * t212 * t223 + f64x8::splat(8.0) * t135 * t219 + t227 * t228;
            let t231 = t215 * t204;
            let t232 = f64x8::splat(1.0) / t231;
            let t234 = t198 / t204 + t214 * t216 + t230 * t232;
            let t235 = t197 * t234;
            let tzk0 = t158 + t235;
            acc_zk = tzk0;
            let t237 = f64x8::splat(1.0) / t14 / v_rho;
            let t238 = t237 * t16;
            let t239 = t20 * t46;
            let t242 = f64x8::splat(0.0011073470983333333) * t13 * t238 * t239;
            let t243 = t42 * t42;
            let t244 = f64x8::splat(1.0) / t243;
            let t245 = t24 * t244;
            let t248 = f64x8::splat(1.0) / t25 * t7 * t9;
            let t249 = t12 * t237;
            let t250 = t16 * t20;
            let t251 = t249 * t250;
            let t252 = t248 * t251;
            let t254 = t238 * t20;
            let t255 = t13 * t254;
            let t257 = ((t22).sqrt());
            let t259 = t257 * t7 * t9;
            let t260 = t259 * t251;
            let t262 = t102 * t36;
            let t264 = t33 * t262 * t38;
            let t266 = -f64x8::splat(0.632975) * t252 - f64x8::splat(0.29896666666666666) * t255 - f64x8::splat(0.1023875) * t260 - f64x8::splat(0.08215666666666667) * t264;
            let t267 = f64x8::splat(1.0) / t45;
            let t268 = t266 * t267;
            let t270 = f64x8::splat(1.0) * t245 * t268;
            let t271 = t20 * t70;
            let t275 = t66 * t66;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t61 * t276;
            let t282 = -f64x8::splat(1.176575) * t252 - f64x8::splat(0.516475) * t255 - f64x8::splat(0.2103875) * t260 - f64x8::splat(0.104195) * t264;
            let t283 = f64x8::splat(1.0) / t69;
            let t284 = t282 * t283;
            let t287 = t20 * t83;
            let t291 = t79 * t79;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t74 * t292;
            let t298 = -f64x8::splat(0.8630833333333333) * t252 - f64x8::splat(0.301925) * t255 - f64x8::splat(0.05501625) * t260 - f64x8::splat(0.082785) * t264;
            let t299 = f64x8::splat(1.0) / t82;
            let t300 = t298 * t299;
            let t305 = t59 * t10;
            let t306 = t250 * t83;
            let t310 = t59 * t74;
            let t312 = t292 * t298 * t299;
            let t318 = ((t5).select(f64x8::splat(0.0), t6 * (t242 + t270 + t59 * (f64x8::splat(0.0005323764196666666) * t13 * t238 * t271 + f64x8::splat(1.0) * t277 * t284 - t242 - t270 + f64x8::splat(0.00018311447306006544) * t13 * t238 * t287 + f64x8::splat(0.5848223622634646) * t293 * t300) - f64x8::splat(0.00018311447306006544) * t305 * t249 * t306 - f64x8::splat(0.5848223622634646) * t310 * t312) / f64x8::splat(2.0)));
            let t319 = t318 * t149;
            let t320 = t319 * t156;
            let t322 = t94 * t126;
            let t323 = t96 * v_rho;
            let t325 = f64x8::splat(1.0) / t34 / t323;
            let t326 = t95 * t325;
            let t328 = t100 * t98;
            let t330 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t326 - f64x8::splat(10.0) / f64x8::splat(3.0) * t328;
            let t331 = param_alpha_ss * t330;
            let t333 = t36 * t325;
            let t336 = t121 * v_tau;
            let t339 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t118 * t333 - f64x8::splat(10.0) / f64x8::splat(3.0) * t336 * t119;
            let t341 = t124 * t147;
            let t344 = t131 * t96;
            let t346 = f64x8::splat(1.0) / t14 / t344;
            let t347 = t16 * t346;
            let t353 = t135 * v_tau;
            let t356 = t142 * t122;
            let t359 = -f64x8::splat(32.0) / f64x8::splat(3.0) * t130 * t347 - f64x8::splat(8.0) / f64x8::splat(3.0) * t139 * t333 * t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t139 * t353 - f64x8::splat(20.0) / f64x8::splat(3.0) * t356 * t328;
            let t361 = t125 * t125;
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = t145 * t362;
            let t366 = t126 * t339 + t147 * t359 - t322 * t331 - f64x8::splat(2.0) * t331 * t341 - f64x8::splat(3.0) * t331 * t363;
            let t367 = t93 * t366;
            let t368 = t367 * t156;
            let t370 = f64x8::splat(1.0) / t96;
            let t371 = v_sigma * t370;
            let t372 = t371 * t153;
            let t373 = t150 * t372;
            let t378 = t171 * t171;
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = t162 * t379;
            let t382 = f64x8::splat(1.0) / t163 * t7;
            let t383 = t9 * t12;
            let t384 = t383 * t237;
            let t385 = t382 * t384;
            let t387 = t10 * t249;
            let t389 = ((t160).sqrt());
            let t390 = t389 * t7;
            let t391 = t390 * t384;
            let t394 = t32 * t11 * t102;
            let t396 = -f64x8::splat(0.632975) * t385 - f64x8::splat(0.29896666666666666) * t387 - f64x8::splat(0.1023875) * t391 - f64x8::splat(0.08215666666666667) * t394;
            let t397 = f64x8::splat(1.0) / t174;
            let t398 = t396 * t397;
            let t401 = t181 * t7;
            let t406 = t181 * t183;
            let t407 = t188 * t188;
            let t408 = f64x8::splat(1.0) / t407;
            let t413 = -f64x8::splat(0.8630833333333333) * t385 - f64x8::splat(0.301925) * t387 - f64x8::splat(0.05501625) * t391 - f64x8::splat(0.082785) * t394;
            let t415 = f64x8::splat(1.0) / t191;
            let t416 = t408 * t413 * t415;
            let t420 = f64x8::splat(0.0011073470983333333) * t10 * t249 * t175 + f64x8::splat(1.0) * t380 * t398 - f64x8::splat(0.00018311447306006544) * t401 * t383 * t237 * t192 - f64x8::splat(0.5848223622634646) * t406 * t416 - f64x8::splat(2.0) * t318;
            let t421 = t420 * t234;
            let t422 = t198 * t216;
            let t425 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t326 - f64x8::splat(20.0) / f64x8::splat(3.0) * t328;
            let t426 = param_alpha_ab * t425;
            let t430 = t211 * v_tau;
            let t433 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t208 * t333 - f64x8::splat(20.0) / f64x8::splat(3.0) * t430 * t119;
            let t435 = t214 * t232;
            let t445 = t227 * t212;
            let t448 = -f64x8::splat(128.0) / f64x8::splat(3.0) * t219 * t347 - f64x8::splat(16.0) / f64x8::splat(3.0) * t223 * t333 * t212 - f64x8::splat(80.0) / f64x8::splat(3.0) * t223 * t353 - f64x8::splat(40.0) / f64x8::splat(3.0) * t445 * t328;
            let t450 = t215 * t215;
            let t451 = f64x8::splat(1.0) / t450;
            let t452 = t230 * t451;
            let t455 = t216 * t433 + t232 * t448 - t422 * t426 - f64x8::splat(2.0) * t426 * t435 - f64x8::splat(3.0) * t426 * t452;
            let t456 = t197 * t455;
            let tvrho0 = t158 + t235 + v_rho * (f64x8::splat(2.0) * t320 + f64x8::splat(2.0) * t368 + t373 / f64x8::splat(4.0) + t421 + t456);
            acc_vrho = tvrho0;
            let t459 = param_alpha_ss * t36;
            let t460 = t459 * t98;
            let t461 = t322 * t460;
            let t462 = t117 * t36;
            let t463 = t98 * t126;
            let t465 = t341 * t460;
            let t467 = t128 * v_sigma;
            let t470 = t138 * t36;
            let t473 = t122 * t470 * t98 + f64x8::splat(4.0) * t135 * t467;
            let t475 = t363 * t460;
            let t477 = t147 * t473 + t462 * t463 - t461 - f64x8::splat(2.0) * t465 - f64x8::splat(3.0) * t475;
            let t478 = t93 * t477;
            let t480 = f64x8::splat(2.0) * t478 * t156;
            let t481 = t151 * t153;
            let t483 = t150 * t481 / f64x8::splat(4.0);
            let t484 = param_alpha_ab * t36;
            let t485 = t484 * t98;
            let t486 = t422 * t485;
            let t488 = t207 * t36;
            let t489 = t98 * t216;
            let t492 = t435 * t485;
            let t494 = t218 * v_sigma;
            let t497 = t222 * t36;
            let t501 = f64x8::splat(2.0) * t212 * t497 * t98 + f64x8::splat(16.0) * t135 * t494;
            let t503 = t452 * t485;
            let t505 = t232 * t501 + f64x8::splat(2.0) * t488 * t489 - f64x8::splat(2.0) * t486 - f64x8::splat(4.0) * t492 - f64x8::splat(6.0) * t503;
            let t506 = t197 * t505;
            let tvsigma0 = v_rho * (t480 - t483 + t506);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t508 = t459 * t102;
            let t511 = t121 * t36;
            let t518 = f64x8::splat(1.0) / t14 / t131;
            let t519 = t16 * t518;
            let t523 = f64x8::splat(4.0) * t139 * t519 + f64x8::splat(4.0) * t262 * t356;
            let t527 = f64x8::splat(2.0) * t102 * t126 * t511 + t147 * t523 - f64x8::splat(2.0) * t322 * t508 - f64x8::splat(4.0) * t341 * t508 - f64x8::splat(6.0) * t363 * t508;
            let t528 = t93 * t527;
            let t530 = f64x8::splat(2.0) * t528 * t156;
            let t531 = v_tau * v_tau;
            let t532 = f64x8::splat(1.0) / t531;
            let t533 = t152 * t532;
            let t535 = t150 * t533 / f64x8::splat(4.0);
            let t536 = t484 * t102;
            let t539 = t211 * t36;
            let t549 = f64x8::splat(16.0) * t223 * t519 + f64x8::splat(8.0) * t262 * t445;
            let t553 = f64x8::splat(4.0) * t102 * t216 * t539 + t232 * t549 - f64x8::splat(4.0) * t422 * t536 - f64x8::splat(8.0) * t435 * t536 - f64x8::splat(12.0) * t452 * t536;
            let t554 = t197 * t553;
            let tvtau0 = v_rho * (t530 + t535 + t554);
            acc_vtau = tvtau0;
            let t562 = f64x8::splat(1.0) / t14 / t96;
            let t563 = t562 * t16;
            let t566 = f64x8::splat(0.0014764627977777779) * t13 * t563 * t239;
            let t569 = t250 * t244 * t266 * t267;
            let t571 = f64x8::splat(0.035616666666666665) * t387 * t569;
            let t572 = t243 * t42;
            let t573 = f64x8::splat(1.0) / t572;
            let t574 = t24 * t573;
            let t575 = t266 * t266;
            let t576 = t575 * t267;
            let t578 = f64x8::splat(2.0) * t574 * t576;
            let t582 = f64x8::splat(1.0) / t25 / t22 * t30 * t31;
            let t583 = t11 * t98;
            let t584 = t36 * t38;
            let t585 = t583 * t584;
            let t586 = t582 * t585;
            let t588 = t12 * t562;
            let t589 = t588 * t250;
            let t590 = t248 * t589;
            let t592 = t563 * t20;
            let t593 = t13 * t592;
            let t595 = f64x8::splat(1.0)/((t22).sqrt());
            let t597 = t595 * t30 * t31;
            let t598 = t597 * t585;
            let t600 = t259 * t589;
            let t603 = t33 * t119 * t38;
            let t605 = -f64x8::splat(0.4219833333333333) * t586 + f64x8::splat(0.8439666666666666) * t590 + f64x8::splat(0.3986222222222222) * t593 + f64x8::splat(0.06825833333333334) * t598 + f64x8::splat(0.13651666666666668) * t600 + f64x8::splat(0.1369277777777778) * t603;
            let t608 = f64x8::splat(1.0) * t245 * t605 * t267;
            let t609 = t243 * t243;
            let t610 = f64x8::splat(1.0) / t609;
            let t611 = t24 * t610;
            let t612 = t45 * t45;
            let t613 = f64x8::splat(1.0) / t612;
            let t614 = t575 * t613;
            let t616 = f64x8::splat(16.081979498692537) * t611 * t614;
            let t622 = t250 * t276 * t282 * t283;
            let t625 = t275 * t66;
            let t626 = f64x8::splat(1.0) / t625;
            let t627 = t61 * t626;
            let t628 = t282 * t282;
            let t629 = t628 * t283;
            let t638 = -f64x8::splat(0.7843833333333333) * t586 + f64x8::splat(1.5687666666666666) * t590 + f64x8::splat(0.6886333333333333) * t593 + f64x8::splat(0.14025833333333335) * t598 + f64x8::splat(0.2805166666666667) * t600 + f64x8::splat(0.17365833333333333) * t603;
            let t642 = t275 * t275;
            let t643 = f64x8::splat(1.0) / t642;
            let t644 = t61 * t643;
            let t645 = t69 * t69;
            let t646 = f64x8::splat(1.0) / t645;
            let t647 = t628 * t646;
            let t653 = t250 * t312;
            let t656 = t291 * t79;
            let t657 = f64x8::splat(1.0) / t656;
            let t658 = t74 * t657;
            let t659 = t298 * t298;
            let t660 = t659 * t299;
            let t669 = -f64x8::splat(0.5753888888888888) * t586 + f64x8::splat(1.1507777777777777) * t590 + f64x8::splat(0.4025666666666667) * t593 + f64x8::splat(0.0366775) * t598 + f64x8::splat(0.073355) * t600 + f64x8::splat(0.137975) * t603;
            let t670 = t669 * t299;
            let t673 = t291 * t291;
            let t674 = f64x8::splat(1.0) / t673;
            let t675 = t74 * t674;
            let t676 = t82 * t82;
            let t677 = f64x8::splat(1.0) / t676;
            let t678 = t659 * t677;
            let t681 = -f64x8::splat(0.0007098352262222222) * t13 * t563 * t271 - f64x8::splat(0.03424666666666667) * t387 * t622 - f64x8::splat(2.0) * t627 * t629 + f64x8::splat(1.0) * t277 * t638 * t283 + f64x8::splat(32.16395899738507) * t644 * t647 + t566 + t571 + t578 - t608 - t616 - f64x8::splat(0.00024415263074675396) * t13 * t563 * t287 - f64x8::splat(0.01084358130030174) * t387 * t653 - f64x8::splat(1.1696447245269292) * t658 * t660 + f64x8::splat(0.5848223622634646) * t293 * t670 + f64x8::splat(17.315859105681465) * t675 * t678;
            let t686 = t59 * t13;
            let t691 = t657 * t659 * t299;
            let t695 = t292 * t669 * t299;
            let t698 = t674 * t659;
            let t699 = t698 * t677;
            let t702 = -t566 - t571 - t578 + t608 + t616 + t59 * t681 + f64x8::splat(0.00024415263074675396) * t305 * t588 * t306 + f64x8::splat(0.01084358130030174) * t686 * t254 * t312 + f64x8::splat(1.1696447245269292) * t310 * t691 - f64x8::splat(0.5848223622634646) * t310 * t695 - f64x8::splat(17.315859105681465) * t310 * t699;
            let t705 = ((t5).select(f64x8::splat(0.0), t6 * t702 / f64x8::splat(2.0)));
            let t706 = t705 * t149;
            let t707 = t706 * t156;
            let t709 = t318 * t366;
            let t710 = t709 * t156;
            let t712 = t319 * t372;
            let t714 = t94 * t147;
            let t715 = param_alpha_ss * param_alpha_ss;
            let t716 = t330 * t330;
            let t717 = t715 * t716;
            let t721 = f64x8::splat(1.0) / t34 / t131;
            let t722 = t95 * t721;
            let t724 = t100 * t325;
            let t726 = f64x8::splat(88.0) / f64x8::splat(9.0) * t722 + f64x8::splat(80.0) / f64x8::splat(9.0) * t724;
            let t727 = param_alpha_ss * t726;
            let t729 = t36 * t721;
            let t734 = f64x8::splat(88.0) / f64x8::splat(9.0) * t118 * t729 + f64x8::splat(80.0) / f64x8::splat(9.0) * t336 * t333;
            let t736 = t339 * t147;
            let t739 = t124 * t362;
            let t744 = t131 * t323;
            let t746 = f64x8::splat(1.0) / t14 / t744;
            let t747 = t16 * t746;
            let t753 = t347 * v_tau;
            let t756 = t142 * t531;
            let t761 = f64x8::splat(608.0) / f64x8::splat(9.0) * t130 * t747 + f64x8::splat(88.0) / f64x8::splat(9.0) * t139 * t729 * t122 + f64x8::splat(160.0) / f64x8::splat(3.0) * t139 * t753 + f64x8::splat(400.0) / f64x8::splat(9.0) * t756 * t135 + f64x8::splat(160.0) / f64x8::splat(9.0) * t356 * t724;
            let t763 = t359 * t362;
            let t767 = f64x8::splat(1.0) / t361 / t114;
            let t768 = t145 * t767;
            let t773 = t126 * t734 + t147 * t761 - t322 * t727 - f64x8::splat(4.0) * t331 * t736 - f64x8::splat(6.0) * t331 * t763 - f64x8::splat(2.0) * t341 * t727 - f64x8::splat(3.0) * t363 * t727 + f64x8::splat(2.0) * t714 * t717 + f64x8::splat(6.0) * t717 * t739 + f64x8::splat(12.0) * t717 * t768;
            let t774 = t93 * t773;
            let t775 = t774 * t156;
            let t777 = t367 * t372;
            let t779 = f64x8::splat(1.0) / t323;
            let t780 = v_sigma * t779;
            let t781 = t780 * t153;
            let t782 = t150 * t781;
            let t787 = t237 * t379;
            let t791 = t378 * t171;
            let t792 = f64x8::splat(1.0) / t791;
            let t793 = t162 * t792;
            let t794 = t396 * t396;
            let t795 = t794 * t397;
            let t800 = f64x8::splat(1.0) / t163 / t160 * t30;
            let t801 = t31 * t11;
            let t802 = t801 * t98;
            let t803 = t800 * t802;
            let t805 = t383 * t562;
            let t806 = t382 * t805;
            let t808 = t10 * t588;
            let t810 = f64x8::splat(1.0)/((t160).sqrt());
            let t811 = t810 * t30;
            let t812 = t811 * t802;
            let t814 = t390 * t805;
            let t816 = t32 * t583;
            let t818 = -f64x8::splat(0.4219833333333333) * t803 + f64x8::splat(0.8439666666666666) * t806 + f64x8::splat(0.3986222222222222) * t808 + f64x8::splat(0.06825833333333334) * t812 + f64x8::splat(0.13651666666666668) * t814 + f64x8::splat(0.1369277777777778) * t816;
            let t819 = t818 * t397;
            let t822 = t378 * t378;
            let t823 = f64x8::splat(1.0) / t822;
            let t824 = t162 * t823;
            let t825 = t174 * t174;
            let t826 = f64x8::splat(1.0) / t825;
            let t827 = t794 * t826;
            let t834 = t181 * t10;
            let t838 = t407 * t188;
            let t839 = f64x8::splat(1.0) / t838;
            let t840 = t413 * t413;
            let t842 = t839 * t840 * t415;
            let t851 = -f64x8::splat(0.5753888888888888) * t803 + f64x8::splat(1.1507777777777777) * t806 + f64x8::splat(0.4025666666666667) * t808 + f64x8::splat(0.0366775) * t812 + f64x8::splat(0.073355) * t814 + f64x8::splat(0.137975) * t816;
            let t853 = t408 * t851 * t415;
            let t856 = t407 * t407;
            let t857 = f64x8::splat(1.0) / t856;
            let t858 = t857 * t840;
            let t859 = t191 * t191;
            let t860 = f64x8::splat(1.0) / t859;
            let t861 = t858 * t860;
            let t865 = -f64x8::splat(0.0014764627977777779) * t10 * t588 * t175 - f64x8::splat(0.035616666666666665) * t13 * t787 * t398 - f64x8::splat(2.0) * t793 * t795 + f64x8::splat(1.0) * t380 * t819 + f64x8::splat(16.081979498692537) * t824 * t827 + f64x8::splat(0.00024415263074675396) * t401 * t383 * t562 * t192 + f64x8::splat(0.01084358130030174) * t834 * t249 * t416 + f64x8::splat(1.1696447245269292) * t406 * t842 - f64x8::splat(0.5848223622634646) * t406 * t853 - f64x8::splat(17.315859105681465) * t406 * t861 - f64x8::splat(2.0) * t705;
            let t866 = t865 * t234;
            let t867 = t420 * t455;
            let t869 = t198 * t232;
            let t870 = param_alpha_ab * param_alpha_ab;
            let t871 = t425 * t425;
            let t872 = t870 * t871;
            let t877 = f64x8::splat(176.0) / f64x8::splat(9.0) * t722 + f64x8::splat(160.0) / f64x8::splat(9.0) * t724;
            let t878 = param_alpha_ab * t877;
            let t884 = f64x8::splat(176.0) / f64x8::splat(9.0) * t208 * t729 + f64x8::splat(160.0) / f64x8::splat(9.0) * t430 * t333;
            let t886 = t433 * t232;
            let t889 = t214 * t451;
            let t901 = t227 * t531;
            let t906 = f64x8::splat(2432.0) / f64x8::splat(9.0) * t219 * t747 + f64x8::splat(176.0) / f64x8::splat(9.0) * t223 * t729 * t212 + f64x8::splat(640.0) / f64x8::splat(3.0) * t223 * t753 + f64x8::splat(1600.0) / f64x8::splat(9.0) * t901 * t135 + f64x8::splat(320.0) / f64x8::splat(9.0) * t445 * t724;
            let t908 = t448 * t451;
            let t912 = f64x8::splat(1.0) / t450 / t204;
            let t913 = t230 * t912;
            let t918 = t216 * t884 + t232 * t906 - t422 * t878 - f64x8::splat(4.0) * t426 * t886 - f64x8::splat(6.0) * t426 * t908 - f64x8::splat(2.0) * t435 * t878 - f64x8::splat(3.0) * t452 * t878 + f64x8::splat(2.0) * t869 * t872 + f64x8::splat(6.0) * t872 * t889 + f64x8::splat(12.0) * t872 * t913;
            let t919 = t197 * t918;
            let tv2rho20 = f64x8::splat(4.0) * t320 + f64x8::splat(4.0) * t368 + t373 / f64x8::splat(2.0) + f64x8::splat(2.0) * t421 + f64x8::splat(2.0) * t456 + v_rho * (f64x8::splat(2.0) * t707 + f64x8::splat(4.0) * t710 + t712 / f64x8::splat(2.0) + f64x8::splat(2.0) * t775 + t777 / f64x8::splat(2.0) - t782 / f64x8::splat(2.0) + t866 + f64x8::splat(2.0) * t867 + t919);
            acc_v2rho2 = tv2rho20;
            let t922 = t318 * t477;
            let t923 = t922 * t156;
            let t925 = t714 * t715;
            let t926 = t119 * t330;
            let t927 = t925 * t926;
            let t929 = t459 * t325;
            let t930 = t322 * t929;
            let t932 = t325 * t126;
            let t935 = t462 * t98;
            let t936 = t147 * param_alpha_ss;
            let t937 = t936 * t330;
            let t940 = t736 * t460;
            let t942 = t739 * t715;
            let t943 = t942 * t926;
            let t945 = t341 * t929;
            let t952 = t138 * t16;
            let t953 = t134 * v_tau;
            let t956 = -f64x8::splat(64.0) / f64x8::splat(3.0) * t467 * t347 - f64x8::splat(8.0) / f64x8::splat(3.0) * t470 * t325 * t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t952 * t953;
            let t958 = t473 * t362;
            let t961 = t763 * t460;
            let t963 = t768 * t715;
            let t964 = t963 * t926;
            let t966 = t363 * t929;
            let t968 = f64x8::splat(2.0) * t927 + f64x8::splat(8.0) / f64x8::splat(3.0) * t930 - f64x8::splat(8.0) / f64x8::splat(3.0) * t462 * t932 - f64x8::splat(2.0) * t935 * t937 - f64x8::splat(2.0) * t940 + f64x8::splat(6.0) * t943 + f64x8::splat(16.0) / f64x8::splat(3.0) * t945 + t956 * t147 - f64x8::splat(3.0) * t958 * t331 - f64x8::splat(3.0) * t961 + f64x8::splat(12.0) * t964 + f64x8::splat(8.0) * t966;
            let t969 = t93 * t968;
            let t970 = t969 * t156;
            let t972 = t478 * t372;
            let t974 = t319 * t481;
            let t976 = t367 * t481;
            let t978 = t370 * t153;
            let t979 = t150 * t978;
            let t981 = t420 * t505;
            let t982 = t869 * t870;
            let t983 = t119 * t425;
            let t984 = t982 * t983;
            let t986 = t484 * t325;
            let t987 = t422 * t986;
            let t989 = t325 * t216;
            let t992 = t488 * t98;
            let t993 = t232 * param_alpha_ab;
            let t994 = t993 * t425;
            let t997 = t886 * t485;
            let t999 = t889 * t870;
            let t1000 = t999 * t983;
            let t1002 = t435 * t986;
            let t1009 = t222 * t16;
            let t1012 = -f64x8::splat(256.0) / f64x8::splat(3.0) * t494 * t347 - f64x8::splat(16.0) / f64x8::splat(3.0) * t497 * t325 * t212 - f64x8::splat(80.0) / f64x8::splat(3.0) * t1009 * t953;
            let t1014 = t501 * t451;
            let t1017 = t908 * t485;
            let t1019 = t913 * t870;
            let t1020 = t1019 * t983;
            let t1022 = t452 * t986;
            let t1024 = f64x8::splat(4.0) * t984 + f64x8::splat(16.0) / f64x8::splat(3.0) * t987 - f64x8::splat(16.0) / f64x8::splat(3.0) * t488 * t989 - f64x8::splat(4.0) * t992 * t994 - f64x8::splat(4.0) * t997 + f64x8::splat(12.0) * t1000 + f64x8::splat(32.0) / f64x8::splat(3.0) * t1002 + t1012 * t232 - f64x8::splat(3.0) * t1014 * t426 - f64x8::splat(6.0) * t1017 + f64x8::splat(24.0) * t1020 + f64x8::splat(16.0) * t1022;
            let t1025 = t197 * t1024;
            let tv2rhosigma0 = t480 - t483 + t506 + v_rho * (f64x8::splat(2.0) * t923 + f64x8::splat(2.0) * t970 + t972 / f64x8::splat(4.0) - t974 / f64x8::splat(4.0) - t976 / f64x8::splat(4.0) + t979 / f64x8::splat(4.0) + t981 + t1025);
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t1028 = t318 * t527;
            let t1029 = t1028 * t156;
            let t1031 = t262 * t330;
            let t1037 = t511 * t102;
            let t1047 = t142 * v_tau;
            let t1052 = -f64x8::splat(52.0) / f64x8::splat(3.0) * t139 * t135 - f64x8::splat(80.0) / f64x8::splat(3.0) * t1047 * t519 - f64x8::splat(20.0) / f64x8::splat(3.0) * t356 * t119;
            let t1054 = t523 * t362;
            let t1062 = f64x8::splat(4.0) * t925 * t1031 + f64x8::splat(10.0) / f64x8::splat(3.0) * t461 - f64x8::splat(10.0) / f64x8::splat(3.0) * t511 * t463 - f64x8::splat(4.0) * t1037 * t937 - f64x8::splat(4.0) * t736 * t508 + f64x8::splat(12.0) * t942 * t1031 + f64x8::splat(20.0) / f64x8::splat(3.0) * t465 + t1052 * t147 - f64x8::splat(3.0) * t1054 * t331 - f64x8::splat(6.0) * t763 * t508 + f64x8::splat(24.0) * t963 * t1031 + f64x8::splat(10.0) * t475;
            let t1063 = t93 * t1062;
            let t1064 = t1063 * t156;
            let t1066 = t528 * t372;
            let t1068 = t319 * t533;
            let t1070 = t367 * t533;
            let t1072 = t371 * t532;
            let t1073 = t150 * t1072;
            let t1075 = t420 * t553;
            let t1076 = t262 * t425;
            let t1082 = t539 * t102;
            let t1092 = t227 * v_tau;
            let t1097 = -f64x8::splat(208.0) / f64x8::splat(3.0) * t223 * t135 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1092 * t519 - f64x8::splat(40.0) / f64x8::splat(3.0) * t445 * t119;
            let t1099 = t549 * t451;
            let t1107 = f64x8::splat(8.0) * t982 * t1076 + f64x8::splat(20.0) / f64x8::splat(3.0) * t486 - f64x8::splat(20.0) / f64x8::splat(3.0) * t539 * t489 - f64x8::splat(8.0) * t1082 * t994 - f64x8::splat(8.0) * t886 * t536 + f64x8::splat(24.0) * t999 * t1076 + f64x8::splat(40.0) / f64x8::splat(3.0) * t492 + t1097 * t232 - f64x8::splat(3.0) * t1099 * t426 - f64x8::splat(12.0) * t908 * t536 + f64x8::splat(48.0) * t1019 * t1076 + f64x8::splat(20.0) * t503;
            let t1108 = t197 * t1107;
            let tv2rhotau0 = t530 + t535 + t554 + v_rho * (f64x8::splat(2.0) * t1029 + f64x8::splat(2.0) * t1064 + t1066 / f64x8::splat(4.0) + t1068 / f64x8::splat(4.0) + t1070 / f64x8::splat(4.0) - t1073 / f64x8::splat(4.0) + t1075 + t1108);
            acc_v2rhotau = tv2rhotau0;
            let t1111 = t715 * t16;
            let t1112 = t1111 * t134;
            let t1113 = t714 * t1112;
            let t1115 = t117 * t16;
            let t1116 = t134 * t147;
            let t1117 = t1116 * param_alpha_ss;
            let t1118 = t1115 * t1117;
            let t1120 = t739 * t1112;
            let t1122 = t128 * t16;
            let t1125 = t958 * t460;
            let t1127 = t768 * t1112;
            let t1129 = f64x8::splat(4.0) * t1116 * t1122 + f64x8::splat(4.0) * t1113 - f64x8::splat(8.0) * t1118 + f64x8::splat(12.0) * t1120 - f64x8::splat(6.0) * t1125 + f64x8::splat(24.0) * t1127;
            let t1130 = t93 * t1129;
            let t1132 = f64x8::splat(2.0) * t1130 * t156;
            let t1134 = t478 * t481 / f64x8::splat(2.0);
            let t1135 = t870 * t16;
            let t1136 = t1135 * t134;
            let t1137 = t869 * t1136;
            let t1139 = t207 * t16;
            let t1140 = t134 * t232;
            let t1141 = t1140 * param_alpha_ab;
            let t1142 = t1139 * t1141;
            let t1144 = t889 * t1136;
            let t1146 = t218 * t16;
            let t1149 = t1014 * t485;
            let t1151 = t913 * t1136;
            let t1153 = f64x8::splat(16.0) * t1140 * t1146 + f64x8::splat(16.0) * t1137 - f64x8::splat(32.0) * t1142 + f64x8::splat(48.0) * t1144 - f64x8::splat(12.0) * t1149 + f64x8::splat(96.0) * t1151;
            let t1154 = t197 * t1153;
            let tv2sigma20 = v_rho * (t1132 - t1134 + t1154);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t1156 = t1111 * t518;
            let t1157 = t714 * t1156;
            let t1159 = t121 * t16;
            let t1160 = t518 * t147;
            let t1161 = t1160 * param_alpha_ss;
            let t1162 = t1159 * t1161;
            let t1166 = t739 * t1156;
            let t1170 = t1054 * t460;
            let t1174 = t768 * t1156;
            let t1176 = -f64x8::splat(8.0) * t1115 * t1161 + f64x8::splat(4.0) * t1160 * t952 - f64x8::splat(6.0) * t508 * t958 + f64x8::splat(8.0) * t1157 - f64x8::splat(8.0) * t1162 + f64x8::splat(24.0) * t1166 - f64x8::splat(3.0) * t1170 + f64x8::splat(48.0) * t1174;
            let t1177 = t93 * t1176;
            let t1179 = f64x8::splat(2.0) * t1177 * t156;
            let t1181 = t528 * t481 / f64x8::splat(4.0);
            let t1183 = t478 * t533 / f64x8::splat(4.0);
            let t1184 = t151 * t532;
            let t1186 = t150 * t1184 / f64x8::splat(4.0);
            let t1187 = t1135 * t518;
            let t1188 = t869 * t1187;
            let t1190 = t211 * t16;
            let t1191 = t518 * t232;
            let t1192 = t1191 * param_alpha_ab;
            let t1193 = t1190 * t1192;
            let t1197 = t889 * t1187;
            let t1201 = t1099 * t485;
            let t1205 = t913 * t1187;
            let t1207 = f64x8::splat(16.0) * t1009 * t1191 - f64x8::splat(12.0) * t1014 * t536 - f64x8::splat(32.0) * t1139 * t1192 + f64x8::splat(32.0) * t1188 - f64x8::splat(32.0) * t1193 + f64x8::splat(96.0) * t1197 - f64x8::splat(6.0) * t1201 + f64x8::splat(192.0) * t1205;
            let t1208 = t197 * t1207;
            let tv2sigmatau0 = v_rho * (t1179 - t1181 + t1183 + t1186 + t1208);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1211 = f64x8::splat(1.0) / t14 / t323;
            let t1212 = t1111 * t1211;
            let t1215 = t1211 * t147;
            let t1221 = t142 * t16;
            let t1228 = -f64x8::splat(32.0) * t1159 * t1215 * param_alpha_ss - f64x8::splat(12.0) * t1054 * t508 + f64x8::splat(16.0) * t1212 * t714 + f64x8::splat(48.0) * t1212 * t739 + f64x8::splat(96.0) * t1212 * t768 + f64x8::splat(16.0) * t1215 * t1221;
            let t1229 = t93 * t1228;
            let t1231 = f64x8::splat(2.0) * t1229 * t156;
            let t1233 = t528 * t533 / f64x8::splat(2.0);
            let t1235 = f64x8::splat(1.0) / t531 / v_tau;
            let t1236 = t152 * t1235;
            let t1238 = t150 * t1236 / f64x8::splat(2.0);
            let t1239 = t1135 * t1211;
            let t1242 = t1211 * t232;
            let t1248 = t227 * t16;
            let t1255 = -f64x8::splat(128.0) * t1190 * t1242 * param_alpha_ab - f64x8::splat(24.0) * t1099 * t536 + f64x8::splat(64.0) * t1239 * t869 + f64x8::splat(192.0) * t1239 * t889 + f64x8::splat(384.0) * t1239 * t913 + f64x8::splat(64.0) * t1242 * t1248;
            let t1256 = t197 * t1255;
            let tv2tau20 = v_rho * (t1231 + t1233 - t1238 + t1256);
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
