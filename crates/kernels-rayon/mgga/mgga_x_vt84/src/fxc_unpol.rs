//! MGGA_X_VT84 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`
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
pub fn mgga_x_vt84_fxc_unpol(
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
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = v_sigma * v_sigma;
            let t22 = t21 * v_sigma;
            let t23 = v_rho * v_rho;
            let t24 = t23 * v_rho;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t22 * t25;
            let t27 = v_tau * v_tau;
            let t28 = t27 * v_tau;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = f64x8::splat(1.0) / t23;
            let t31 = t21 * t30;
            let t32 = f64x8::splat(1.0) / t27;
            let t33 = t31 * t32;
            let t35 = f64x8::splat(1.0) + t33 / f64x8::splat(64.0);
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t29 * t37;
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t26 * t38) * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t43 * t47;
            let t49 = f64x8::splat(M_CBRT2);
            let t50 = t49 * t49;
            let t51 = v_sigma * t50;
            let t52 = t19 * t19;
            let t54 = f64x8::splat(1.0) / t52 / t23;
            let t55 = t51 * t54;
            let t58 = v_tau * t50;
            let t60 = f64x8::splat(1.0) / t52 / v_rho;
            let t63 = t58 * t60 - t55 / f64x8::splat(8.0);
            let t64 = t63 * t42;
            let t67 = f64x8::splat(5.0) / f64x8::splat(9.0) * t64 * t47 - f64x8::splat(1.0);
            let t68 = t47 * t67;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t64 * t68;
            let t72 = ((t71).sqrt());
            let t73 = f64x8::splat(1.0) / t72;
            let t76 = t42 * t47;
            let t77 = t76 * t55;
            let t79 = f64x8::splat(9.0) / f64x8::splat(20.0) * t67 * t73 + t77 / f64x8::splat(36.0);
            let t80 = t79 * t79;
            let t83 = t42 * t42;
            let t85 = f64x8::splat(1.0) / t45 / t44;
            let t86 = t83 * t85;
            let t87 = t21 * t49;
            let t88 = t23 * t23;
            let t89 = t88 * v_rho;
            let t91 = f64x8::splat(1.0) / t19 / t89;
            let t93 = t86 * t87 * t91;
            let t95 = f64x8::splat(162.0) * t33 + f64x8::splat(100.0) * t93;
            let t96 = ((t95).sqrt());
            let t101 = t88 * t88;
            let t102 = f64x8::splat(1.0) / t101;
            let t105 = t48 * t55 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t80 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t79 * t96 + f64x8::splat(5.301186990888923e-05) * t93 + f64x8::splat(0.0019577914932045744) * t33 + f64x8::splat(4.3721079261097765e-06) * t22 * t102;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t77;
            let t108 = t107 * t107;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t105 * t109;
            let t112 = (simd::exp(-f64x8::splat(0.0001863) * t110));
            let t113 = f64x8::splat(1.0) + t110;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t112 * t114;
            let t117 = t105 * t105;
            let t118 = t108 * t108;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = (simd::exp(-f64x8::splat(0.00150903) * t117 * t119));
            let t123 = f64x8::splat(1.0) - t122;
            let t124 = f64x8::splat(1.0) / t105;
            let t127 = f64x8::splat(10.0) / f64x8::splat(81.0) * t124 * t108 - f64x8::splat(1.0);
            let t129 = t110 * t115 + t123 * t127 + f64x8::splat(1.0);
            let t133 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t129));
            let tzk0 = f64x8::splat(2.0) * t133;
            acc_zk = tzk0;
            let t135 = t18 / t52;
            let t139 = f64x8::splat(1.0) / t88;
            let t140 = t22 * t139;
            let t143 = t21 * t21;
            let t144 = t143 * v_sigma;
            let t145 = t88 * t23;
            let t146 = f64x8::splat(1.0) / t145;
            let t147 = t144 * t146;
            let t148 = t27 * t27;
            let t149 = t148 * v_tau;
            let t150 = f64x8::splat(1.0) / t149;
            let t152 = f64x8::splat(1.0) / t36 / t35;
            let t153 = t150 * t152;
            let t157 = (-f64x8::splat(0.01259478515625) * t140 * t38 + f64x8::splat(0.000262391357421875) * t147 * t153) * t42;
            let t158 = t157 * t47;
            let t162 = f64x8::splat(1.0) / t52 / t24;
            let t163 = t51 * t162;
            let t169 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t58 * t54 + t163 / f64x8::splat(3.0);
            let t170 = t169 * t42;
            let t171 = t47 * t73;
            let t175 = f64x8::splat(1.0) / t72 / t71;
            let t176 = t67 * t175;
            let t179 = t63 * t83;
            let t180 = t85 * t169;
            let t183 = f64x8::splat(0.2222222222222222) * t170 * t68 + f64x8::splat(0.12345679012345678) * t179 * t180;
            let t186 = t76 * t163;
            let t188 = t170 * t171 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t183 - f64x8::splat(2.0) / f64x8::splat(27.0) * t186;
            let t193 = f64x8::splat(1.0) / t96;
            let t194 = t79 * t193;
            let t195 = t21 * t25;
            let t196 = t195 * t32;
            let t199 = f64x8::splat(1.0) / t19 / t145;
            let t201 = t86 * t87 * t199;
            let t203 = -f64x8::splat(324.0) * t196 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t201;
            let t208 = t101 * v_rho;
            let t209 = f64x8::splat(1.0) / t208;
            let t212 = t158 * t55 / f64x8::splat(24.0) - t48 * t163 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t188 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t188 * t96 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t194 * t203 - f64x8::splat(0.0002827299728474092) * t201 - f64x8::splat(0.003915582986409149) * t196 - f64x8::splat(3.497686340887821e-05) * t22 * t209;
            let t213 = t212 * t109;
            let t215 = t108 * t107;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t105 * t216;
            let t218 = t217 * t115;
            let t222 = t217 * t42;
            let t223 = t47 * v_sigma;
            let t224 = t50 * t162;
            let t225 = t223 * t224;
            let t226 = t222 * t225;
            let t228 = -f64x8::splat(0.0001863) * t213 - f64x8::splat(5.835784882944196e-05) * t226;
            let t229 = t228 * t112;
            let t230 = t229 * t114;
            let t232 = t113 * t113;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t112 * t233;
            let t236 = t213 + f64x8::splat(0.3132466389127319) * t226;
            let t237 = t234 * t236;
            let t239 = t105 * t119;
            let t242 = t118 * t107;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t117 * t243;
            let t245 = t244 * t42;
            let t248 = -f64x8::splat(0.00301806) * t239 * t212 - f64x8::splat(0.0009453971510369597) * t245 * t225;
            let t249 = t248 * t122;
            let t250 = t249 * t127;
            let t251 = f64x8::splat(1.0) / t117;
            let t252 = t251 * t108;
            let t255 = t124 * t107;
            let t256 = t255 * t42;
            let t259 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t252 * t212 - f64x8::splat(0.0386724245571274) * t256 * t225;
            let t261 = t213 * t115 + f64x8::splat(0.3132466389127319) * t218 * t186 + t110 * t230 - t110 * t237 - t250 + t123 * t259;
            let t266 = ((t3).select(f64x8::splat(0.0), -t7 * t135 * t129 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t261));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t266 + f64x8::splat(2.0) * t133;
            acc_vrho = tvrho0;
            let t271 = f64x8::splat(1.0) / t89;
            let t272 = t143 * t271;
            let t276 = (f64x8::splat(0.01259478515625) * t195 * t38 - f64x8::splat(0.000262391357421875) * t272 * t153) * t42;
            let t277 = t276 * t47;
            let t280 = t47 * t50;
            let t281 = t280 * t54;
            let t284 = t50 * t54;
            let t285 = t76 * t73;
            let t286 = t284 * t285;
            let t288 = t76 * t67;
            let t289 = t284 * t288;
            let t291 = t85 * t50;
            let t293 = t179 * t291 * t54;
            let t295 = -f64x8::splat(0.027777777777777776) * t289 - f64x8::splat(0.015432098765432098) * t293;
            let t298 = t284 * t76;
            let t300 = -t286 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t295 + t298 / f64x8::splat(36.0);
            let t305 = v_sigma * t30;
            let t306 = t305 * t32;
            let t308 = v_sigma * t49;
            let t310 = t86 * t308 * t91;
            let t312 = f64x8::splat(324.0) * t306 + f64x8::splat(200.0) * t310;
            let t319 = t277 * t55 / f64x8::splat(24.0) + t43 * t281 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t300 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t300 * t96 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t194 * t312 + f64x8::splat(0.00010602373981777846) * t310 + f64x8::splat(0.003915582986409149) * t306 + f64x8::splat(1.311632377832933e-05) * t21 * t102;
            let t320 = t319 * t109;
            let t325 = t217 * t50;
            let t326 = t54 * t42;
            let t327 = t326 * t47;
            let t328 = t325 * t327;
            let t330 = -f64x8::splat(0.0001863) * t320 + f64x8::splat(2.1884193311040734e-05) * t328;
            let t331 = t330 * t112;
            let t332 = t331 * t114;
            let t335 = t320 - f64x8::splat(0.11746748959227447) * t328;
            let t336 = t234 * t335;
            let t340 = t244 * t50;
            let t343 = -f64x8::splat(0.00301806) * t239 * t319 + f64x8::splat(0.0003545239316388599) * t340 * t327;
            let t344 = t343 * t122;
            let t345 = t344 * t127;
            let t348 = t255 * t50;
            let t351 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t252 * t319 + f64x8::splat(0.014502159208922774) * t348 * t327;
            let t353 = t320 * t115 - f64x8::splat(0.11746748959227447) * t218 * t298 + t110 * t332 - t110 * t336 - t345 + t123 * t351;
            let t357 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t353));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t357;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t359 = f64x8::splat(1.0) / t148;
            let t360 = t359 * t37;
            let t363 = t144 * t271;
            let t364 = t148 * t27;
            let t365 = f64x8::splat(1.0) / t364;
            let t366 = t365 * t152;
            let t370 = (-f64x8::splat(0.01259478515625) * t26 * t360 + f64x8::splat(0.000262391357421875) * t363 * t366) * t42;
            let t371 = t370 * t47;
            let t374 = t50 * t60;
            let t382 = f64x8::splat(0.2222222222222222) * t374 * t288 + f64x8::splat(0.12345679012345678) * t179 * t291 * t60;
            let t385 = t374 * t285 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t382;
            let t390 = t31 * t29;
            let t394 = t371 * t55 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t385 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t385 * t96 + f64x8::splat(73.0) / f64x8::splat(600.0) * t194 * t390 - f64x8::splat(0.003915582986409149) * t390;
            let t395 = t394 * t109;
            let t397 = t394 * t112;
            let t398 = t397 * t114;
            let t401 = t234 * t394;
            let t403 = t394 * t122;
            let t404 = t403 * t127;
            let t407 = t123 * t251;
            let t408 = t108 * t394;
            let t411 = t395 * t115 - f64x8::splat(0.0001863) * t239 * t398 - t239 * t401 + f64x8::splat(0.00301806) * t239 * t404 - f64x8::splat(10.0) / f64x8::splat(81.0) * t407 * t408;
            let t415 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t411));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t415;
            acc_vtau = tvtau0;
            let t418 = t18 * t60;
            let t425 = t22 * t271;
            let t428 = t88 * t24;
            let t429 = f64x8::splat(1.0) / t428;
            let t430 = t144 * t429;
            let t433 = t143 * t22;
            let t434 = t433 * t209;
            let t435 = t148 * t28;
            let t436 = f64x8::splat(1.0) / t435;
            let t437 = t36 * t36;
            let t438 = f64x8::splat(1.0) / t437;
            let t439 = t436 * t438;
            let t443 = (f64x8::splat(0.050379140625) * t425 * t38 - f64x8::splat(0.002361522216796875) * t430 * t153 + f64x8::splat(2.459918975830078e-05) * t434 * t439) * t42;
            let t444 = t443 * t47;
            let t450 = f64x8::splat(1.0) / t52 / t88;
            let t451 = t51 * t450;
            let t454 = t188 * t188;
            let t459 = f64x8::splat(40.0) / f64x8::splat(9.0) * t58 * t162 - f64x8::splat(11.0) / f64x8::splat(9.0) * t451;
            let t460 = t459 * t42;
            let t463 = t47 * t175;
            let t464 = t463 * t183;
            let t467 = t71 * t71;
            let t469 = f64x8::splat(1.0) / t72 / t467;
            let t470 = t67 * t469;
            let t471 = t183 * t183;
            let t476 = t169 * t169;
            let t483 = f64x8::splat(0.2222222222222222) * t460 * t68 + f64x8::splat(0.24691358024691357) * t476 * t83 * t85 + f64x8::splat(0.12345679012345678) * t179 * t85 * t459;
            let t486 = t76 * t451;
            let t488 = t460 * t171 / f64x8::splat(4.0) - t170 * t464 / f64x8::splat(4.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t471 - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t483 + f64x8::splat(22.0) / f64x8::splat(81.0) * t486;
            let t493 = t188 * t193;
            let t497 = f64x8::splat(1.0) / t96 / t95;
            let t498 = t79 * t497;
            let t499 = t203 * t203;
            let t502 = t21 * t139;
            let t503 = t502 * t32;
            let t506 = f64x8::splat(1.0) / t19 / t428;
            let t507 = t87 * t506;
            let t508 = t86 * t507;
            let t510 = f64x8::splat(972.0) * t503 + f64x8::splat(30400.0) / f64x8::splat(9.0) * t508;
            let t515 = t101 * t23;
            let t516 = f64x8::splat(1.0) / t515;
            let t519 = t444 * t55 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t158 * t163 + f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t451 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t454 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t488 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t488 * t96 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t493 * t203 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t498 * t499 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t194 * t510 + f64x8::splat(0.0017906231613669251) * t508 + f64x8::splat(0.011746748959227447) * t503 + f64x8::splat(0.00031479177067990393) * t22 * t516;
            let t520 = t519 * t109;
            let t522 = t212 * t216;
            let t523 = t522 * t115;
            let t530 = t239 * t115;
            let t533 = t217 * t230;
            let t536 = t234 * t42;
            let t537 = t217 * t536;
            let t538 = t224 * t236;
            let t539 = t223 * t538;
            let t545 = t522 * t42;
            let t546 = t545 * t225;
            let t548 = t239 * t83;
            let t549 = t85 * t21;
            let t550 = t49 * t506;
            let t551 = t549 * t550;
            let t552 = t548 * t551;
            let t554 = t50 * t450;
            let t555 = t223 * t554;
            let t556 = t222 * t555;
            let t558 = -f64x8::splat(0.0001863) * t520 - f64x8::splat(0.00011671569765888392) * t546 - f64x8::splat(5.48412e-05) * t552 + f64x8::splat(0.00021397877904128719) * t556;
            let t559 = t558 * t112;
            let t560 = t559 * t114;
            let t562 = t228 * t228;
            let t563 = t562 * t112;
            let t564 = t563 * t114;
            let t566 = t110 * t228;
            let t570 = f64x8::splat(1.0) / t232 / t113;
            let t571 = t112 * t570;
            let t572 = t236 * t236;
            let t573 = t571 * t572;
            let t579 = t520 + f64x8::splat(0.6264932778254638) * t546 + f64x8::splat(0.2943703703703704) * t552 - f64x8::splat(1.1485710093466837) * t556;
            let t580 = t234 * t579;
            let t582 = t212 * t212;
            let t585 = t105 * t243;
            let t586 = t212 * t42;
            let t587 = t585 * t586;
            let t593 = f64x8::splat(1.0) / t118 / t108;
            let t594 = t117 * t593;
            let t595 = t594 * t83;
            let t600 = -f64x8::splat(0.00301806) * t582 * t119 - f64x8::splat(0.0037815886041478388) * t587 * t225 - f64x8::splat(0.00301806) * t239 * t519 - f64x8::splat(0.0014807124) * t595 * t551 + f64x8::splat(0.003466456220468852) * t245 * t555;
            let t601 = t600 * t122;
            let t602 = t601 * t127;
            let t603 = t248 * t248;
            let t604 = t603 * t122;
            let t605 = t604 * t127;
            let t606 = t249 * t259;
            let t608 = t117 * t105;
            let t609 = f64x8::splat(1.0) / t608;
            let t610 = t609 * t108;
            let t613 = t251 * t107;
            let t614 = t613 * t586;
            let t620 = t124 * t83 * t85;
            let t625 = f64x8::splat(20.0) / f64x8::splat(81.0) * t610 * t582 + f64x8::splat(0.0773448491142548) * t614 * t225 - f64x8::splat(10.0) / f64x8::splat(81.0) * t252 * t519 + f64x8::splat(0.012114007011126353) * t620 * t507 + f64x8::splat(0.14179889004280047) * t256 * t555;
            let t627 = t520 * t115 + f64x8::splat(0.6264932778254638) * t523 * t186 + f64x8::splat(2.0) * t213 * t230 - f64x8::splat(2.0) * t213 * t237 + f64x8::splat(0.2943703703703704) * t530 * t508 + f64x8::splat(0.6264932778254638) * t533 * t186 - f64x8::splat(0.6264932778254638) * t537 * t539 - f64x8::splat(1.1485710093466837) * t218 * t486 + t110 * t560 + t110 * t564 - f64x8::splat(2.0) * t566 * t237 + f64x8::splat(2.0) * t110 * t573 - t110 * t580 - t602 - t605 - f64x8::splat(2.0) * t606 + t123 * t625;
            let t632 = ((t3).select(f64x8::splat(0.0), t7 * t418 * t129 / f64x8::splat(12.0) - t7 * t135 * t261 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t627));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t632 + f64x8::splat(4.0) * t266;
            acc_v2rho2 = tv2rho20;
            let t640 = t143 * t146;
            let t643 = t143 * t21;
            let t644 = t643 * t102;
            let t648 = (-f64x8::splat(0.03778435546875) * t502 * t38 + f64x8::splat(0.002099130859375) * t640 * t153 - f64x8::splat(2.459918975830078e-05) * t644 * t439) * t42;
            let t649 = t648 * t47;
            let t656 = t280 * t162;
            let t661 = t224 * t285;
            let t663 = t284 * t42;
            let t664 = t663 * t464;
            let t666 = t463 * t295;
            let t669 = t295 * t183;
            let t672 = t224 * t288;
            let t674 = t86 * t169;
            let t675 = t284 * t674;
            let t678 = t179 * t291 * t162;
            let t680 = f64x8::splat(0.07407407407407407) * t672 - f64x8::splat(0.030864197530864196) * t675 + f64x8::splat(0.0411522633744856) * t678;
            let t683 = t224 * t76;
            let t685 = t661 / f64x8::splat(12.0) + t664 / f64x8::splat(64.0) - t170 * t666 / f64x8::splat(8.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t669 - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t680 - f64x8::splat(2.0) / f64x8::splat(27.0) * t683;
            let t690 = t300 * t193;
            let t695 = t312 * t203;
            let t698 = v_sigma * t25;
            let t699 = t698 * t32;
            let t701 = t308 * t199;
            let t702 = t86 * t701;
            let t704 = -f64x8::splat(648.0) * t699 - f64x8::splat(3200.0) / f64x8::splat(3.0) * t702;
            let t711 = t649 * t55 / f64x8::splat(24.0) - t277 * t163 / f64x8::splat(9.0) + t157 * t281 / f64x8::splat(24.0) - t43 * t656 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t188 * t300 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t685 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t685 * t96 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t690 * t203 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t493 * t312 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t498 * t695 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t194 * t704 - f64x8::splat(0.0005654599456948184) * t702 - f64x8::splat(0.007831165972818297) * t699 - f64x8::splat(0.00010493059022663464) * t21 * t209;
            let t712 = t711 * t109;
            let t714 = t319 * t216;
            let t715 = t714 * t115;
            let t724 = t217 * t229;
            let t725 = t114 * t50;
            let t726 = t725 * t327;
            let t729 = t217 * t234;
            let t730 = t76 * t236;
            let t731 = t284 * t730;
            let t737 = t217 * t332;
            let t741 = t714 * t42;
            let t742 = t741 * t225;
            let t744 = t522 * t50;
            let t745 = t744 * t327;
            let t747 = t239 * t49;
            let t748 = t199 * t83;
            let t749 = t85 * v_sigma;
            let t750 = t748 * t749;
            let t751 = t747 * t750;
            let t753 = t162 * t42;
            let t754 = t753 * t47;
            let t755 = t325 * t754;
            let t757 = -f64x8::splat(0.0001863) * t712 - f64x8::splat(5.835784882944196e-05) * t742 + f64x8::splat(2.1884193311040734e-05) * t745 + f64x8::splat(2.056545e-05) * t751 - f64x8::splat(5.835784882944196e-05) * t755;
            let t758 = t757 * t112;
            let t759 = t758 * t114;
            let t761 = t712 * t115 + f64x8::splat(0.3132466389127319) * t715 * t186 + t320 * t230 - t320 * t237 - f64x8::splat(0.11746748959227447) * t523 * t298 - f64x8::splat(0.11038888888888888) * t530 * t702 - f64x8::splat(0.11746748959227447) * t724 * t726 + f64x8::splat(0.11746748959227447) * t729 * t731 + f64x8::splat(0.3132466389127319) * t218 * t683 + t213 * t332 + f64x8::splat(0.3132466389127319) * t737 * t186 + t110 * t759;
            let t762 = t110 * t330;
            let t766 = t217 * t336;
            let t770 = t110 * t112;
            let t771 = t570 * t335;
            let t772 = t771 * t236;
            let t779 = t712 + f64x8::splat(0.3132466389127319) * t742 - f64x8::splat(0.11746748959227447) * t745 - f64x8::splat(0.11038888888888888) * t751 + f64x8::splat(0.3132466389127319) * t755;
            let t780 = t234 * t779;
            let t782 = t212 * t119;
            let t785 = t319 * t42;
            let t786 = t585 * t785;
            let t791 = t585 * t50;
            let t792 = t47 * t212;
            let t793 = t326 * t792;
            let t796 = t594 * t49;
            let t801 = -f64x8::splat(0.00301806) * t782 * t319 - f64x8::splat(0.0018907943020739194) * t786 * t225 - f64x8::splat(0.00301806) * t239 * t711 + f64x8::splat(0.0007090478632777198) * t791 * t793 + f64x8::splat(0.00055526715) * t796 * t750 - f64x8::splat(0.0009453971510369597) * t340 * t754;
            let t802 = t801 * t122;
            let t803 = t802 * t127;
            let t804 = t343 * t248;
            let t805 = t122 * t127;
            let t806 = t804 * t805;
            let t807 = t344 * t259;
            let t808 = t249 * t351;
            let t809 = t319 * t212;
            let t812 = t613 * t785;
            let t817 = t613 * t50;
            let t824 = f64x8::splat(20.0) / f64x8::splat(81.0) * t610 * t809 + f64x8::splat(0.0386724245571274) * t812 * t225 - f64x8::splat(10.0) / f64x8::splat(81.0) * t252 * t711 - f64x8::splat(0.014502159208922774) * t817 * t793 - f64x8::splat(0.004542752629172382) * t620 * t701 - f64x8::splat(0.0386724245571274) * t348 * t754;
            let t826 = t762 * t230 - t762 * t237 - t213 * t336 - f64x8::splat(0.3132466389127319) * t766 * t186 - t566 * t336 + f64x8::splat(2.0) * t770 * t772 - t110 * t780 - t803 - t806 - t807 - t808 + t123 * t824;
            let t827 = t761 + t826;
            let t832 = ((t3).select(f64x8::splat(0.0), -t7 * t135 * t353 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t827));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t832 + f64x8::splat(2.0) * t357;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t842 = t433 * t102;
            let t843 = t148 * t148;
            let t844 = f64x8::splat(1.0) / t843;
            let t845 = t844 * t438;
            let t849 = (f64x8::splat(0.03778435546875) * t140 * t360 - f64x8::splat(0.002099130859375) * t147 * t366 + f64x8::splat(2.459918975830078e-05) * t842 * t845) * t42;
            let t850 = t849 * t47;
            let t858 = t374 * t42;
            let t861 = t463 * t382;
            let t864 = t382 * t183;
            let t871 = -f64x8::splat(0.37037037037037035) * t289 + f64x8::splat(0.24691358024691357) * t374 * t674 - f64x8::splat(0.205761316872428) * t293;
            let t874 = -f64x8::splat(5.0) / f64x8::splat(12.0) * t286 - t858 * t464 / f64x8::splat(8.0) - t170 * t861 / f64x8::splat(8.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t864 - f64x8::splat(9.0) / f64x8::splat(40.0) * t176 * t871;
            let t879 = t385 * t193;
            let t884 = t498 * t21;
            let t885 = t30 * t29;
            let t886 = t885 * t203;
            let t889 = t195 * t29;
            let t893 = t850 * t55 / f64x8::splat(24.0) - t371 * t163 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t188 * t385 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t874 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t874 * t96 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t879 * t203 + f64x8::splat(73.0) / f64x8::splat(600.0) * t493 * t390 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t884 * t886 - f64x8::splat(73.0) / f64x8::splat(300.0) * t194 * t889 + f64x8::splat(0.007831165972818297) * t889;
            let t894 = t893 * t109;
            let t896 = t394 * t216;
            let t897 = t896 * t115;
            let t904 = t585 * t398;
            let t907 = t893 * t112;
            let t908 = t907 * t114;
            let t911 = t239 * t394;
            let t917 = t585 * t401;
            let t920 = t894 * t115 + f64x8::splat(0.3132466389127319) * t897 * t186 + t395 * t230 - t395 * t237 - f64x8::splat(0.0001863) * t782 * t398 - f64x8::splat(0.00011671569765888392) * t904 * t186 - f64x8::splat(0.0001863) * t239 * t908 - f64x8::splat(0.0001863) * t911 * t230 + f64x8::splat(0.0001863) * t911 * t237 - t782 * t401 - f64x8::splat(0.6264932778254638) * t917 * t186;
            let t921 = t239 * t228;
            let t923 = t239 * t112;
            let t924 = t570 * t394;
            let t925 = t924 * t236;
            let t928 = t234 * t893;
            let t932 = t585 * t404;
            let t935 = t893 * t122;
            let t936 = t935 * t127;
            let t941 = t403 * t259;
            let t944 = t252 * t394;
            let t947 = t123 * t609;
            let t948 = t408 * t212;
            let t951 = t107 * t394;
            let t952 = t407 * t951;
            let t955 = t108 * t893;
            let t958 = -t921 * t401 + f64x8::splat(2.0) * t923 * t925 - t239 * t928 + f64x8::splat(0.00301806) * t782 * t404 + f64x8::splat(0.0018907943020739194) * t932 * t186 + f64x8::splat(0.00301806) * t239 * t936 + f64x8::splat(0.00301806) * t911 * t250 + f64x8::splat(0.00301806) * t239 * t941 + f64x8::splat(10.0) / f64x8::splat(81.0) * t249 * t944 + f64x8::splat(20.0) / f64x8::splat(81.0) * t947 * t948 + f64x8::splat(0.0386724245571274) * t952 * t186 - f64x8::splat(10.0) / f64x8::splat(81.0) * t407 * t955;
            let t959 = t920 + t958;
            let t964 = ((t3).select(f64x8::splat(0.0), -t7 * t135 * t411 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t959));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t964 + f64x8::splat(2.0) * t415;
            acc_v2rhotau = tv2rhotau0;
            let t974 = (f64x8::splat(0.0251895703125) * t698 * t38 - f64x8::splat(0.001836739501953125) * t425 * t153 + f64x8::splat(2.459918975830078e-05) * t430 * t439) * t42;
            let t975 = t974 * t47;
            let t980 = t300 * t300;
            let t982 = t663 * t666;
            let t984 = t295 * t295;
            let t987 = t176 * t49;
            let t988 = t91 * t83;
            let t989 = t988 * t85;
            let t990 = t987 * t989;
            let t992 = t982 / f64x8::splat(32.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t984 - f64x8::splat(0.001736111111111111) * t990;
            let t999 = t312 * t312;
            let t1002 = t30 * t32;
            let t1004 = t49 * t91;
            let t1005 = t1004 * t86;
            let t1007 = f64x8::splat(324.0) * t1002 + f64x8::splat(200.0) * t1005;
            let t1014 = t975 * t55 / f64x8::splat(24.0) + t276 * t281 / f64x8::splat(12.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t980 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t992 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t992 * t96 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t690 * t312 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t498 * t999 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t194 * t1007 + f64x8::splat(0.00010602373981777846) * t1005 + f64x8::splat(0.003915582986409149) * t1002 + f64x8::splat(2.623264755665866e-05) * v_sigma * t102;
            let t1015 = t1014 * t109;
            let t1025 = t217 * t331;
            let t1028 = t76 * t335;
            let t1029 = t284 * t1028;
            let t1033 = t714 * t50;
            let t1034 = t1033 * t327;
            let t1036 = t747 * t989;
            let t1038 = -f64x8::splat(0.0001863) * t1015 + f64x8::splat(4.376838662208147e-05) * t1034 - f64x8::splat(7.71204375e-06) * t1036;
            let t1039 = t1038 * t112;
            let t1040 = t1039 * t114;
            let t1042 = t330 * t330;
            let t1043 = t1042 * t112;
            let t1044 = t1043 * t114;
            let t1048 = t335 * t335;
            let t1049 = t571 * t1048;
            let t1054 = t1015 - f64x8::splat(0.23493497918454895) * t1034 + f64x8::splat(0.04139583333333333) * t1036;
            let t1055 = t234 * t1054;
            let t1057 = t319 * t319;
            let t1060 = t585 * t319;
            let t1067 = -f64x8::splat(0.00301806) * t1057 * t119 + f64x8::splat(0.0014180957265554396) * t1060 * t298 - f64x8::splat(0.00301806) * t239 * t1014 - f64x8::splat(0.00020822518125) * t796 * t989;
            let t1068 = t1067 * t122;
            let t1069 = t1068 * t127;
            let t1070 = t343 * t343;
            let t1071 = t1070 * t122;
            let t1072 = t1071 * t127;
            let t1073 = t344 * t351;
            let t1077 = t613 * t319;
            let t1082 = t124 * t49;
            let t1085 = f64x8::splat(20.0) / f64x8::splat(81.0) * t610 * t1057 - f64x8::splat(0.029004318417845548) * t1077 * t298 - f64x8::splat(10.0) / f64x8::splat(81.0) * t252 * t1014 + f64x8::splat(0.0017035322359396433) * t1082 * t989;
            let t1087 = t1015 * t115 - f64x8::splat(0.23493497918454895) * t715 * t298 + f64x8::splat(2.0) * t320 * t332 - f64x8::splat(2.0) * t320 * t336 + f64x8::splat(0.04139583333333333) * t530 * t1005 - f64x8::splat(0.23493497918454895) * t1025 * t726 + f64x8::splat(0.23493497918454895) * t729 * t1029 + t110 * t1040 + t110 * t1044 - f64x8::splat(2.0) * t762 * t336 + f64x8::splat(2.0) * t110 * t1049 - t110 * t1055 - t1069 - t1072 - f64x8::splat(2.0) * t1073 + t123 * t1085;
            let t1091 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1087));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t1091;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t1097 = t643 * t429;
            let t1101 = (-f64x8::splat(0.03778435546875) * t195 * t360 + f64x8::splat(0.002099130859375) * t272 * t366 - f64x8::splat(2.459918975830078e-05) * t1097 * t845) * t42;
            let t1102 = t1101 * t47;
            let t1111 = t663 * t861;
            let t1113 = t382 * t295;
            let t1117 = f64x8::splat(1.0) / t19 / t88;
            let t1118 = t1117 * t83;
            let t1120 = t987 * t1118 * t85;
            let t1122 = -t858 * t666 / f64x8::splat(8.0) + t1111 / f64x8::splat(64.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t1113 + f64x8::splat(0.013888888888888888) * t1120;
            let t1131 = t885 * t312;
            let t1134 = t305 * t29;
            let t1138 = t1102 * t55 / f64x8::splat(24.0) + t370 * t281 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t300 * t385 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t1122 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t1122 * t96 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t879 * t312 + f64x8::splat(73.0) / f64x8::splat(600.0) * t690 * t390 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t884 * t1131 + f64x8::splat(73.0) / f64x8::splat(300.0) * t194 * t1134 - f64x8::splat(0.007831165972818297) * t1134;
            let t1139 = t1138 * t109;
            let t1145 = t319 * t119;
            let t1148 = t585 * t397;
            let t1151 = t1138 * t112;
            let t1152 = t1151 * t114;
            let t1160 = t585 * t234;
            let t1161 = t394 * t50;
            let t1162 = t1161 * t327;
            let t1165 = t1139 * t115 - f64x8::splat(0.11746748959227447) * t897 * t298 + t395 * t332 - t395 * t336 - f64x8::splat(0.0001863) * t1145 * t398 + f64x8::splat(4.376838662208147e-05) * t1148 * t726 - f64x8::splat(0.0001863) * t239 * t1152 - f64x8::splat(0.0001863) * t911 * t332 + f64x8::splat(0.0001863) * t911 * t336 - t1145 * t401 + f64x8::splat(0.23493497918454895) * t1160 * t1162;
            let t1166 = t239 * t330;
            let t1168 = t924 * t335;
            let t1171 = t234 * t1138;
            let t1175 = t585 * t403;
            let t1176 = t127 * t50;
            let t1177 = t1176 * t327;
            let t1180 = t1138 * t122;
            let t1181 = t1180 * t127;
            let t1186 = t403 * t351;
            let t1191 = t408 * t319;
            let t1196 = t108 * t1138;
            let t1199 = -t1166 * t401 + f64x8::splat(2.0) * t923 * t1168 - t239 * t1171 + f64x8::splat(0.00301806) * t1145 * t404 - f64x8::splat(0.0007090478632777198) * t1175 * t1177 + f64x8::splat(0.00301806) * t239 * t1181 + f64x8::splat(0.00301806) * t911 * t345 + f64x8::splat(0.00301806) * t239 * t1186 + f64x8::splat(10.0) / f64x8::splat(81.0) * t344 * t944 + f64x8::splat(20.0) / f64x8::splat(81.0) * t947 * t1191 - f64x8::splat(0.014502159208922774) * t952 * t298 - f64x8::splat(10.0) / f64x8::splat(81.0) * t407 * t1196;
            let t1200 = t1165 + t1199;
            let t1204 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1200));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t1204;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1206 = t150 * t37;
            let t1209 = t436 * t152;
            let t1212 = t433 * t429;
            let t1214 = f64x8::splat(1.0) / t843 / v_tau;
            let t1215 = t1214 * t438;
            let t1219 = (f64x8::splat(0.050379140625) * t26 * t1206 - f64x8::splat(0.002361522216796875) * t363 * t1209 + f64x8::splat(2.459918975830078e-05) * t1212 * t1215) * t42;
            let t1220 = t1219 * t47;
            let t1223 = t385 * t385;
            let t1227 = t382 * t382;
            let t1231 = t49 * t85;
            let t1233 = f64x8::splat(1.0) / t19 / t24;
            let t1237 = -t858 * t861 / f64x8::splat(4.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t470 * t1227 - f64x8::splat(0.1111111111111111) * t176 * t83 * t1231 * t1233;
            let t1244 = t143 * t139;
            let t1245 = t1244 * t365;
            let t1248 = t31 * t359;
            let t1252 = t1220 * t55 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t1223 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t79 * t1237 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t1237 * t96 + f64x8::splat(73.0) / f64x8::splat(300.0) * t879 * t390 + f64x8::splat(1971.0) / f64x8::splat(100.0) * t498 * t1245 - f64x8::splat(73.0) / f64x8::splat(200.0) * t194 * t1248 + f64x8::splat(0.011746748959227447) * t1248;
            let t1253 = t1252 * t109;
            let t1255 = t394 * t394;
            let t1256 = t1255 * t119;
            let t1261 = t1252 * t112;
            let t1262 = t1261 * t114;
            let t1265 = t105 * t593;
            let t1266 = t1255 * t112;
            let t1267 = t1266 * t114;
            let t1270 = t1266 * t233;
            let t1273 = t571 * t1255;
            let t1276 = t234 * t1252;
            let t1280 = t1252 * t122;
            let t1281 = t1280 * t127;
            let t1284 = t118 * t118;
            let t1285 = f64x8::splat(1.0) / t1284;
            let t1286 = t117 * t1285;
            let t1287 = t1255 * t122;
            let t1288 = t1287 * t127;
            let t1291 = t124 * t109;
            let t1294 = t108 * t1255;
            let t1297 = t108 * t1252;
            let t1300 = t1253 * t115 - f64x8::splat(0.0003726) * t1256 * t115 - f64x8::splat(2.0) * t1256 * t234 - f64x8::splat(0.0001863) * t239 * t1262 + f64x8::splat(3.470769e-08) * t1265 * t1267 + f64x8::splat(0.0003726) * t1265 * t1270 + f64x8::splat(2.0) * t1265 * t1273 - t239 * t1276 + f64x8::splat(0.00301806) * t1256 * t805 + f64x8::splat(0.00301806) * t239 * t1281 - f64x8::splat(9.1086861636e-06) * t1286 * t1288 - f64x8::splat(0.0007452) * t1291 * t1287 + f64x8::splat(20.0) / f64x8::splat(81.0) * t947 * t1294 - f64x8::splat(10.0) / f64x8::splat(81.0) * t407 * t1297;
            let t1304 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1300));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t1304;
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
