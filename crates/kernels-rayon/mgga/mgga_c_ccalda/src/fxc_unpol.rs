//! MGGA_C_CCALDA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
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
pub fn mgga_c_ccalda_fxc_unpol(
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
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c = f64x8::splat(param_c);
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
            let t2 = f64x8::splat(1.0) + param_c;
            let t3 = (simd::cbrt(v_rho));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / v_rho;
            let t8 = v_rho * v_rho;
            let t10 = f64x8::splat(1.0) / t4 / t8;
            let t13 = v_tau * t6 - v_sigma * t10 / f64x8::splat(8.0);
            let t14 = t2 * t13;
            let t15 = f64x8::splat(M_CBRT6);
            let t16 = t14 * t15;
            let t17 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t18 = (simd::cbrt(t17));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t26 = t15 * t20 * t22;
            let t29 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c * t13 * t26;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = f64x8::splat(M_CBRT3);
            let t32 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t31 * t33;
            let t35 = f64x8::splat(M_CBRT4);
            let t36 = t35 * t35;
            let t39 = t34 * t36 / t3;
            let t41 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t39;
            let t42 = ((t39).sqrt());
            let t45 = ((t39) * (t39).sqrt());
            let t47 = t31 * t31;
            let t48 = t33 * t33;
            let t49 = t47 * t48;
            let t52 = t49 * t35 / t4;
            let t54 = f64x8::splat(3.79785) * t42 + f64x8::splat(0.8969) * t39 + f64x8::splat(0.204775) * t45 + f64x8::splat(0.123235) * t52;
            let t57 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t54;
            let t58 = (simd::ln(t57));
            let t62 = (simd::cbrt(zeta_threshold));
            let t64 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t62 * zeta_threshold, f64x8::splat(1.0)));
            let t70 = (f64x8::splat(2.0) * t64 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t21 - f64x8::splat(2.0));
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t39;
            let t77 = f64x8::splat(5.1785) * t42 + f64x8::splat(0.905775) * t39 + f64x8::splat(0.1100325) * t45 + f64x8::splat(0.1241775) * t52;
            let t80 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t77;
            let t81 = (simd::ln(t80));
            let t85 = -f64x8::splat(0.0621814) * t41 * t58 + f64x8::splat(0.0197516734986138) * t70 * t72 * t81;
            let t87 = t23 * t30 * t85;
            let t89 = f64x8::splat(5.0) / f64x8::splat(9.0) * t16 * t87;
            let t90 = t23 * t30;
            let t93 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t16 * t90;
            let t94 = t93 * t85;
            let tzk0 = t89 + t94;
            acc_zk = tzk0;
            let t97 = t8 * v_rho;
            let t99 = f64x8::splat(1.0) / t4 / t97;
            let t102 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t10 + v_sigma * t99 / f64x8::splat(3.0);
            let t103 = t2 * t102;
            let t104 = t103 * t15;
            let t105 = t104 * t87;
            let t107 = t15 * t15;
            let t109 = f64x8::splat(1.0) / t18 / t17;
            let t110 = t107 * t109;
            let t111 = t14 * t110;
            let t112 = t29 * t29;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t21 * t113;
            let t115 = t85 * param_c;
            let t117 = t114 * t115 * t102;
            let t118 = t111 * t117;
            let t121 = f64x8::splat(1.0) / t3 / v_rho;
            let t122 = t36 * t121;
            let t126 = t54 * t54;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t41 * t127;
            let t130 = f64x8::splat(1.0) / t42 * t31;
            let t131 = t33 * t36;
            let t132 = t131 * t121;
            let t133 = t130 * t132;
            let t135 = t34 * t122;
            let t137 = ((t39).sqrt());
            let t138 = t137 * t31;
            let t139 = t138 * t132;
            let t142 = t49 * t35 * t6;
            let t144 = -f64x8::splat(0.632975) * t133 - f64x8::splat(0.29896666666666666) * t135 - f64x8::splat(0.1023875) * t139 - f64x8::splat(0.08215666666666667) * t142;
            let t145 = f64x8::splat(1.0) / t57;
            let t146 = t144 * t145;
            let t149 = t70 * t31;
            let t154 = t70 * t72;
            let t155 = t77 * t77;
            let t156 = f64x8::splat(1.0) / t155;
            let t161 = -f64x8::splat(0.8630833333333333) * t133 - f64x8::splat(0.301925) * t135 - f64x8::splat(0.05501625) * t139 - f64x8::splat(0.082785) * t142;
            let t163 = f64x8::splat(1.0) / t80;
            let t164 = t156 * t161 * t163;
            let t167 = f64x8::splat(0.0011073470983333333) * t34 * t122 * t58 + f64x8::splat(1.0) * t128 * t146 - f64x8::splat(0.00018311447306006544) * t149 * t131 * t121 * t81 - f64x8::splat(0.5848223622634646) * t154 * t164;
            let t169 = t23 * t30 * t167;
            let t170 = t16 * t169;
            let t175 = t114 * param_c * t102;
            let t178 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t104 * t90 + f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t175;
            let t179 = t178 * t85;
            let t180 = t93 * t167;
            let tvrho0 = t89 + t94 + v_rho * (f64x8::splat(5.0) / f64x8::splat(9.0) * t105 - f64x8::splat(50.0) / f64x8::splat(81.0) * t118 + f64x8::splat(5.0) / f64x8::splat(9.0) * t170 + t179 + t180);
            acc_vrho = tvrho0;
            let t183 = t2 * t10;
            let t184 = t183 * t15;
            let t185 = t184 * t87;
            let t186 = f64x8::splat(5.0) / f64x8::splat(72.0) * t185;
            let t189 = t111 * t114 * t115 * t10;
            let t190 = f64x8::splat(25.0) / f64x8::splat(324.0) * t189;
            let t191 = t184 * t90;
            let t195 = t111 * t114 * param_c * t10;
            let t197 = f64x8::splat(5.0) / f64x8::splat(72.0) * t191 - f64x8::splat(25.0) / f64x8::splat(324.0) * t195;
            let t198 = t197 * t85;
            let tvsigma0 = v_rho * (-t186 + t190 + t198);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t200 = t2 * t6;
            let t201 = t200 * t15;
            let t203 = f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t87;
            let t207 = f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t114 * t115 * t6;
            let t214 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t90 + f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t114 * param_c * t6;
            let t215 = t214 * t85;
            let tvtau0 = v_rho * (t203 - t207 + t215);
            acc_vtau = tvtau0;
            let t224 = t8 * t8;
            let t226 = f64x8::splat(1.0) / t4 / t224;
            let t229 = f64x8::splat(40.0) / f64x8::splat(9.0) * v_tau * t99 - f64x8::splat(11.0) / f64x8::splat(9.0) * v_sigma * t226;
            let t230 = t2 * t229;
            let t231 = t230 * t15;
            let t232 = t231 * t87;
            let t234 = t102 * t102;
            let t235 = t2 * t234;
            let t236 = t235 * t110;
            let t237 = t114 * t115;
            let t238 = t236 * t237;
            let t240 = t104 * t169;
            let t242 = t17 * t17;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t14 * t243;
            let t246 = f64x8::splat(1.0) / t112 / t29;
            let t247 = t246 * t85;
            let t248 = param_c * param_c;
            let t249 = t248 * t234;
            let t250 = t247 * t249;
            let t251 = t244 * t250;
            let t253 = t167 * param_c;
            let t255 = t114 * t253 * t102;
            let t256 = t111 * t255;
            let t259 = t114 * t115 * t229;
            let t260 = t111 * t259;
            let t263 = f64x8::splat(1.0) / t3 / t8;
            let t264 = t36 * t263;
            let t268 = t34 * t36;
            let t269 = t121 * t127;
            let t273 = t126 * t54;
            let t274 = f64x8::splat(1.0) / t273;
            let t275 = t41 * t274;
            let t276 = t144 * t144;
            let t277 = t276 * t145;
            let t282 = f64x8::splat(1.0) / t42 / t39 * t47;
            let t283 = t48 * t35;
            let t284 = t283 * t10;
            let t285 = t282 * t284;
            let t287 = t131 * t263;
            let t288 = t130 * t287;
            let t290 = t34 * t264;
            let t292 = f64x8::splat(1.0)/((t39).sqrt());
            let t293 = t292 * t47;
            let t294 = t293 * t284;
            let t296 = t138 * t287;
            let t299 = t49 * t35 * t10;
            let t301 = -f64x8::splat(0.4219833333333333) * t285 + f64x8::splat(0.8439666666666666) * t288 + f64x8::splat(0.3986222222222222) * t290 + f64x8::splat(0.06825833333333334) * t294 + f64x8::splat(0.13651666666666668) * t296 + f64x8::splat(0.1369277777777778) * t299;
            let t302 = t301 * t145;
            let t305 = t126 * t126;
            let t306 = f64x8::splat(1.0) / t305;
            let t307 = t41 * t306;
            let t308 = t57 * t57;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t276 * t309;
            let t317 = t70 * t34;
            let t321 = t155 * t77;
            let t322 = f64x8::splat(1.0) / t321;
            let t323 = t161 * t161;
            let t325 = t322 * t323 * t163;
            let t334 = -f64x8::splat(0.5753888888888888) * t285 + f64x8::splat(1.1507777777777777) * t288 + f64x8::splat(0.4025666666666667) * t290 + f64x8::splat(0.0366775) * t294 + f64x8::splat(0.073355) * t296 + f64x8::splat(0.137975) * t299;
            let t336 = t156 * t334 * t163;
            let t339 = t155 * t155;
            let t340 = f64x8::splat(1.0) / t339;
            let t341 = t340 * t323;
            let t342 = t80 * t80;
            let t343 = f64x8::splat(1.0) / t342;
            let t344 = t341 * t343;
            let t347 = -f64x8::splat(0.0014764627977777779) * t34 * t264 * t58 - f64x8::splat(0.035616666666666665) * t268 * t269 * t146 - f64x8::splat(2.0) * t275 * t277 + f64x8::splat(1.0) * t128 * t302 + f64x8::splat(16.081979498692537) * t307 * t310 + f64x8::splat(0.00024415263074675396) * t149 * t131 * t263 * t81 + f64x8::splat(0.01084358130030174) * t317 * t122 * t164 + f64x8::splat(1.1696447245269292) * t154 * t325 - f64x8::splat(0.5848223622634646) * t154 * t336 - f64x8::splat(17.315859105681465) * t154 * t344;
            let t349 = t23 * t30 * t347;
            let t350 = t16 * t349;
            let t355 = t109 * t21;
            let t357 = t355 * t113 * param_c;
            let t360 = t246 * t248;
            let t361 = t360 * t234;
            let t365 = t114 * param_c * t229;
            let t368 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t231 * t90 + f64x8::splat(100.0) / f64x8::splat(81.0) * t235 * t107 * t357 - f64x8::splat(2000.0) / f64x8::splat(243.0) * t244 * t361 + f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t365;
            let t369 = t368 * t85;
            let t370 = t178 * t167;
            let t372 = t93 * t347;
            let tv2rho20 = f64x8::splat(10.0) / f64x8::splat(9.0) * t105 - f64x8::splat(100.0) / f64x8::splat(81.0) * t118 + f64x8::splat(10.0) / f64x8::splat(9.0) * t170 + f64x8::splat(2.0) * t179 + f64x8::splat(2.0) * t180 + v_rho * (f64x8::splat(5.0) / f64x8::splat(9.0) * t232 - f64x8::splat(100.0) / f64x8::splat(81.0) * t238 + f64x8::splat(10.0) / f64x8::splat(9.0) * t240 + f64x8::splat(2000.0) / f64x8::splat(243.0) * t251 - f64x8::splat(100.0) / f64x8::splat(81.0) * t256 - f64x8::splat(50.0) / f64x8::splat(81.0) * t260 + f64x8::splat(5.0) / f64x8::splat(9.0) * t350 + t369 + f64x8::splat(2.0) * t370 + t372);
            acc_v2rho2 = tv2rho20;
            let t375 = t2 * t99;
            let t376 = t375 * t15;
            let t377 = t376 * t87;
            let t379 = t183 * t110;
            let t380 = t379 * t117;
            let t382 = t184 * t169;
            let t384 = t243 * t246;
            let t385 = t14 * t384;
            let t386 = t85 * t248;
            let t387 = t10 * t102;
            let t389 = t385 * t386 * t387;
            let t393 = t111 * t114 * t253 * t10;
            let t397 = t111 * t114 * t115 * t99;
            let t399 = t376 * t90;
            let t401 = t379 * t175;
            let t404 = t244 * t360 * t387;
            let t408 = t111 * t114 * param_c * t99;
            let t410 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t399 - f64x8::splat(25.0) / f64x8::splat(162.0) * t401 + f64x8::splat(250.0) / f64x8::splat(243.0) * t404 + f64x8::splat(50.0) / f64x8::splat(243.0) * t408;
            let t411 = t410 * t85;
            let t412 = t197 * t167;
            let tv2rhosigma0 = -t186 + t190 + t198 + v_rho * (f64x8::splat(5.0) / f64x8::splat(27.0) * t377 + f64x8::splat(25.0) / f64x8::splat(162.0) * t380 - f64x8::splat(5.0) / f64x8::splat(72.0) * t382 - f64x8::splat(250.0) / f64x8::splat(243.0) * t389 + f64x8::splat(25.0) / f64x8::splat(324.0) * t393 - f64x8::splat(50.0) / f64x8::splat(243.0) * t397 + t411 + t412);
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t416 = t200 * t110;
            let t417 = t416 * t117;
            let t419 = t201 * t169;
            let t421 = t6 * t102;
            let t423 = t385 * t386 * t421;
            let t427 = t111 * t114 * t253 * t6;
            let t437 = f64x8::splat(25.0) / f64x8::splat(27.0) * t191 + f64x8::splat(100.0) / f64x8::splat(81.0) * t416 * t175 - f64x8::splat(2000.0) / f64x8::splat(243.0) * t244 * t360 * t421 - f64x8::splat(250.0) / f64x8::splat(243.0) * t195;
            let t438 = t437 * t85;
            let t439 = t214 * t167;
            let tv2rhotau0 = t203 - t207 + t215 + v_rho * (-f64x8::splat(25.0) / f64x8::splat(27.0) * t185 - f64x8::splat(100.0) / f64x8::splat(81.0) * t417 + f64x8::splat(5.0) / f64x8::splat(9.0) * t419 + f64x8::splat(2000.0) / f64x8::splat(243.0) * t423 - f64x8::splat(50.0) / f64x8::splat(81.0) * t427 + f64x8::splat(250.0) / f64x8::splat(243.0) * t189 + t438 + t439);
            acc_v2rhotau = tv2rhotau0;
            let t442 = t224 * v_rho;
            let t444 = f64x8::splat(1.0) / t3 / t442;
            let t445 = t2 * t444;
            let t446 = t445 * t110;
            let t447 = t446 * t237;
            let t448 = f64x8::splat(25.0) / f64x8::splat(1296.0) * t447;
            let t449 = t248 * t444;
            let t451 = t244 * t247 * t449;
            let t452 = f64x8::splat(125.0) / f64x8::splat(972.0) * t451;
            let t454 = t445 * t107 * t357;
            let t457 = t244 * t360 * t444;
            let t459 = f64x8::splat(25.0) / f64x8::splat(1296.0) * t454 - f64x8::splat(125.0) / f64x8::splat(972.0) * t457;
            let t460 = t459 * t85;
            let tv2sigma20 = v_rho * (-t448 + t452 + t460);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t463 = f64x8::splat(1.0) / t3 / t224;
            let t464 = t2 * t463;
            let t465 = t464 * t110;
            let t466 = t465 * t237;
            let t467 = f64x8::splat(25.0) / f64x8::splat(162.0) * t466;
            let t468 = t248 * t463;
            let t470 = t244 * t247 * t468;
            let t471 = f64x8::splat(250.0) / f64x8::splat(243.0) * t470;
            let t473 = t464 * t107 * t357;
            let t476 = t244 * t360 * t463;
            let t478 = -f64x8::splat(25.0) / f64x8::splat(162.0) * t473 + f64x8::splat(250.0) / f64x8::splat(243.0) * t476;
            let t479 = t478 * t85;
            let tv2sigmatau0 = v_rho * (t467 - t471 + t479);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t482 = f64x8::splat(1.0) / t3 / t97;
            let t483 = t2 * t482;
            let t484 = t483 * t110;
            let t486 = f64x8::splat(100.0) / f64x8::splat(81.0) * t484 * t237;
            let t487 = t248 * t482;
            let t490 = f64x8::splat(2000.0) / f64x8::splat(243.0) * t244 * t247 * t487;
            let t497 = f64x8::splat(100.0) / f64x8::splat(81.0) * t483 * t107 * t357 - f64x8::splat(2000.0) / f64x8::splat(243.0) * t244 * t360 * t482;
            let t498 = t497 * t85;
            let tv2tau20 = v_rho * (-t486 + t490 + t498);
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
