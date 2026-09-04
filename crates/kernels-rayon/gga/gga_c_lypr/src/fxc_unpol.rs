//! GGA_C_LYPR fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`
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
pub fn gga_c_lypr_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_m1: f64,
    param_omega: f64,
    param_d: f64,
    param_m2: f64,
    param_b: f64,
    param_c: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_m1 = f64x8::splat(param_m1);
    let param_omega = f64x8::splat(param_omega);
    let param_d = f64x8::splat(param_d);
    let param_m2 = f64x8::splat(param_m2);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_a = f64x8::splat(param_a);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (simd::cbrt(v_rho));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = (simd::erfc(param_m1 * param_omega * t3));
            let t7 = param_d * t3 + f64x8::splat(1.0);
            let t8 = f64x8::splat(1.0) / t7;
            let t10 = param_m2 * param_omega;
            let t12 = (simd::erfc(t10 * t3));
            let t13 = t12 * param_b;
            let t15 = (simd::exp(-param_c * t3));
            let t16 = t15 * t8;
            let t17 = v_rho * v_rho;
            let t18 = t2 * t2;
            let t20 = f64x8::splat(1.0) / t18 / t17;
            let t21 = v_sigma * t20;
            let t23 = param_d * t8 + param_c;
            let t24 = t23 * t3;
            let t26 = -f64x8::splat(1.0) / f64x8::splat(72.0) - f64x8::splat(7.0) / f64x8::splat(72.0) * t24;
            let t28 = f64x8::splat(M_CBRT3);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = zeta_threshold * zeta_threshold;
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * t36;
            let t39 = ((t34).select(t37 * t35, f64x8::splat(1.0)));
            let t43 = f64x8::splat(5.0) / f64x8::splat(2.0) - t24 / f64x8::splat(18.0);
            let t44 = t43 * v_sigma;
            let t45 = t20 * t39;
            let t48 = t24 - f64x8::splat(11.0);
            let t49 = t48 * v_sigma;
            let t52 = ((t34).select(t37 * t35 * zeta_threshold, f64x8::splat(1.0)));
            let t53 = t20 * t52;
            let t56 = f64x8::splat(M_CBRT2);
            let t57 = t56 * t56;
            let t58 = v_sigma * t57;
            let t61 = ((t34).select(t35, f64x8::splat(1.0)));
            let t62 = t61 * v_sigma;
            let t64 = t57 * t20 * t39;
            let t70 = -t21 * t26 - f64x8::splat(3.0) / f64x8::splat(10.0) * t29 * t32 * t39 + t44 * t45 / f64x8::splat(8.0) + t49 * t53 / f64x8::splat(144.0) - t56 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * t45 - t62 * t64 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t71 = t16 * t70;
            let t73 = param_b * t15;
            let t74 = ((f64x8::splat(M_PI)).sqrt());
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t73 * t76;
            let t78 = param_m2 * param_m2;
            let t79 = param_omega * param_omega;
            let t81 = f64x8::splat(1.0) / t18;
            let t83 = (simd::exp(-t78 * t79 * t81));
            let t84 = t17 * v_rho;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t83 * t85;
            let tzk0 = param_a * (-t5 * t8 + t13 * t71 + f64x8::splat(7.0) / f64x8::splat(36.0) * t77 * t10 * t86 * v_sigma);
            acc_zk = tzk0;
            let t92 = v_rho * param_a;
            let t93 = param_m1 * param_m1;
            let t96 = (simd::exp(-t93 * t79 * t81));
            let t98 = t75 * t96 * param_m1;
            let t100 = f64x8::splat(1.0) / t2 / v_rho;
            let t105 = t7 * t7;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t5 * t106;
            let t108 = param_d * t100;
            let t111 = t75 * t83;
            let t112 = t111 * t10;
            let t113 = t100 * param_b;
            let t117 = t13 * param_c;
            let t118 = t100 * t15;
            let t119 = t8 * t70;
            let t123 = t13 * t15;
            let t124 = t106 * t70;
            let t129 = f64x8::splat(1.0) / t18 / t84;
            let t130 = v_sigma * t129;
            let t133 = param_d * param_d;
            let t134 = t133 * t106;
            let t139 = -t134 / t18 / v_rho + t23 * t100;
            let t140 = f64x8::splat(7.0) / f64x8::splat(216.0) * t139;
            let t142 = t139 / f64x8::splat(54.0);
            let t143 = t142 * v_sigma;
            let t146 = t129 * t39;
            let t150 = -t139 / f64x8::splat(3.0);
            let t151 = t150 * v_sigma;
            let t154 = t129 * t52;
            let t160 = t57 * t129 * t39;
            let t166 = f64x8::splat(8.0) / f64x8::splat(3.0) * t130 * t26 - t21 * t140 + t143 * t45 / f64x8::splat(8.0) - t44 * t146 / f64x8::splat(3.0) + t151 * t53 / f64x8::splat(144.0) - t49 * t154 / f64x8::splat(54.0) - t56 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t58 * t146 + f64x8::splat(4.0) / f64x8::splat(3.0) * t62 * t160) / f64x8::splat(8.0);
            let t167 = t16 * t166;
            let t169 = param_b * param_c;
            let t170 = t17 * t17;
            let t172 = f64x8::splat(1.0) / t2 / t170;
            let t173 = t172 * t15;
            let t176 = t75 * param_m2;
            let t177 = param_omega * t83;
            let t179 = t176 * t177 * v_sigma;
            let t182 = t106 * t75;
            let t184 = t73 * t182 * param_m2;
            let t190 = t78 * param_m2;
            let t191 = t79 * param_omega;
            let t192 = t190 * t191;
            let t194 = f64x8::splat(1.0) / t18 / t170;
            let t195 = t194 * t83;
            let t200 = f64x8::splat(1.0) / t170;
            let t201 = t83 * t200;
            let t206 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t98 * param_omega * t100 * t8 - t107 * t108 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t113 * t71 + t117 * t118 * t119 / f64x8::splat(3.0) + t123 * t124 * t108 / f64x8::splat(3.0) + t13 * t167 + f64x8::splat(7.0) / f64x8::splat(108.0) * t169 * t173 * t8 * t179 + f64x8::splat(7.0) / f64x8::splat(108.0) * t184 * t177 * t172 * v_sigma * param_d + f64x8::splat(7.0) / f64x8::splat(54.0) * t77 * t192 * t195 * v_sigma - f64x8::splat(7.0) / f64x8::splat(12.0) * t77 * t10 * t201 * v_sigma;
            let tvrho0 = t92 * t206 + tzk0;
            acc_vrho = tvrho0;
            let t216 = t61 * t57;
            let t222 = -t20 * t26 + t43 * t20 * t39 / f64x8::splat(8.0) + t48 * t20 * t52 / f64x8::splat(144.0) - t56 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t64 - t216 * t45 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t223 = t16 * t222;
            let t228 = t13 * t223 + f64x8::splat(7.0) / f64x8::splat(36.0) * t77 * t10 * t86;
            let tvsigma0 = t92 * t228;
            acc_vsigma = tvsigma0;
            let t231 = v_sigma * t194;
            let t236 = t133 * param_d;
            let t238 = f64x8::splat(1.0) / t105 / t7;
            let t239 = t236 * t238;
            let t240 = t239 * t85;
            let t242 = t134 * t20;
            let t245 = f64x8::splat(1.0) / t2 / t17;
            let t246 = t23 * t245;
            let t248 = -f64x8::splat(7.0) / f64x8::splat(324.0) * t240 + f64x8::splat(7.0) / f64x8::splat(108.0) * t242 - f64x8::splat(7.0) / f64x8::splat(162.0) * t246;
            let t253 = -t240 / f64x8::splat(81.0) + t242 / f64x8::splat(27.0) - f64x8::splat(2.0) / f64x8::splat(81.0) * t246;
            let t254 = t253 * v_sigma;
            let t259 = t194 * t39;
            let t265 = f64x8::splat(2.0) / f64x8::splat(9.0) * t240 - f64x8::splat(2.0) / f64x8::splat(3.0) * t242 + f64x8::splat(4.0) / f64x8::splat(9.0) * t246;
            let t266 = t265 * v_sigma;
            let t271 = t194 * t52;
            let t277 = t57 * t194 * t39;
            let t283 = -f64x8::splat(88.0) / f64x8::splat(9.0) * t231 * t26 + f64x8::splat(16.0) / f64x8::splat(3.0) * t130 * t140 - t21 * t248 + t254 * t45 / f64x8::splat(8.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t143 * t146 + f64x8::splat(11.0) / f64x8::splat(9.0) * t44 * t259 + t266 * t53 / f64x8::splat(144.0) - t151 * t154 / f64x8::splat(27.0) + f64x8::splat(11.0) / f64x8::splat(162.0) * t49 * t271 - t56 * (f64x8::splat(352.0) / f64x8::splat(27.0) * t58 * t259 - f64x8::splat(44.0) / f64x8::splat(9.0) * t62 * t277) / f64x8::splat(8.0);
            let t284 = t16 * t283;
            let t287 = t13 * param_c * t20;
            let t288 = t15 * t106;
            let t290 = t288 * t70 * param_d;
            let t293 = t245 * t15;
            let t297 = param_d * t245;
            let t302 = t106 * param_d;
            let t306 = t8 * t166;
            let t310 = param_c * param_c;
            let t311 = t13 * t310;
            let t312 = t20 * t15;
            let t316 = t106 * t166;
            let t320 = t238 * t70;
            let t321 = t133 * t20;
            let t327 = t5 * t238;
            let t330 = t170 * v_rho;
            let t332 = f64x8::splat(1.0) / t2 / t330;
            let t333 = t332 * t15;
            let t344 = t111 * t10 * t20;
            let t345 = t169 * t71;
            let t348 = t13 * t284 + f64x8::splat(2.0) / f64x8::splat(9.0) * t287 * t290 - f64x8::splat(4.0) / f64x8::splat(9.0) * t117 * t293 * t119 - f64x8::splat(4.0) / f64x8::splat(9.0) * t123 * t124 * t297 - f64x8::splat(4.0) / f64x8::splat(9.0) * t98 * param_omega * t20 * t302 + f64x8::splat(2.0) / f64x8::splat(3.0) * t117 * t118 * t306 + t311 * t312 * t119 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t123 * t316 * t108 + f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t320 * t321 + f64x8::splat(4.0) / f64x8::splat(9.0) * t107 * t297 - f64x8::splat(2.0) / f64x8::splat(9.0) * t327 * t321 - f64x8::splat(77.0) / f64x8::splat(162.0) * t169 * t333 * t8 * t179 - f64x8::splat(77.0) / f64x8::splat(162.0) * t184 * t177 * t332 * v_sigma * param_d + f64x8::splat(4.0) / f64x8::splat(9.0) * t344 * t345;
            let t349 = t124 * param_d;
            let t350 = t73 * t349;
            let t353 = param_b * t310;
            let t355 = f64x8::splat(1.0) / t18 / t330;
            let t356 = t355 * t15;
            let t361 = t170 * t17;
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = t362 * t15;
            let t366 = t75 * t190;
            let t367 = t191 * t83;
            let t369 = t366 * t367 * v_sigma;
            let t372 = t238 * t75;
            let t374 = t73 * t372 * param_m2;
            let t375 = t355 * v_sigma;
            let t381 = t73 * t182 * t190;
            let t383 = t83 * v_sigma;
            let t384 = t383 * param_d;
            let t388 = t93 * param_m1;
            let t390 = t75 * t388 * t191;
            let t399 = t245 * param_b;
            let t403 = t355 * t83;
            let t408 = f64x8::splat(1.0) / t330;
            let t409 = t83 * t408;
            let t418 = t366 * t191 * t85;
            let t419 = t83 * param_b;
            let t420 = t419 * t71;
            let t423 = t78 * t78;
            let t424 = t423 * param_m2;
            let t425 = t79 * t79;
            let t426 = t425 * param_omega;
            let t427 = t424 * t426;
            let t429 = f64x8::splat(1.0) / t2 / t361;
            let t430 = t429 * t83;
            let t436 = t169 * t356 * t106;
            let t437 = t176 * param_omega;
            let t438 = t437 * t384;
            let t441 = f64x8::splat(4.0) / f64x8::splat(9.0) * t344 * t350 + f64x8::splat(7.0) / f64x8::splat(324.0) * t353 * t356 * t8 * t179 + f64x8::splat(7.0) / f64x8::splat(81.0) * t169 * t363 * t8 * t369 + f64x8::splat(7.0) / f64x8::splat(162.0) * t374 * t177 * t375 * t133 + f64x8::splat(7.0) / f64x8::splat(81.0) * t381 * t191 * t362 * t384 - f64x8::splat(4.0) / f64x8::splat(9.0) * t390 * t85 * t96 * t8 + f64x8::splat(8.0) / f64x8::splat(9.0) * t98 * param_omega * t245 * t8 - f64x8::splat(8.0) / f64x8::splat(9.0) * t112 * t399 * t71 - f64x8::splat(161.0) / f64x8::splat(162.0) * t77 * t192 * t403 * v_sigma + f64x8::splat(7.0) / f64x8::splat(3.0) * t77 * t10 * t409 * v_sigma + f64x8::splat(4.0) / f64x8::splat(3.0) * t112 * t113 * t167 + f64x8::splat(4.0) / f64x8::splat(9.0) * t418 * t420 + f64x8::splat(7.0) / f64x8::splat(81.0) * t77 * t427 * t430 * v_sigma + f64x8::splat(7.0) / f64x8::splat(162.0) * t436 * t438;
            let t442 = t348 + t441;
            let tv2rho20 = f64x8::splat(2.0) * param_a * t206 + t92 * t442;
            acc_v2rho2 = tv2rho20;
            let t448 = t8 * t222;
            let t452 = t106 * t222;
            let t477 = f64x8::splat(8.0) / f64x8::splat(3.0) * t129 * t26 - t20 * t140 + t142 * t20 * t39 / f64x8::splat(8.0) - t43 * t129 * t39 / f64x8::splat(3.0) + t150 * t20 * t52 / f64x8::splat(144.0) - t48 * t129 * t52 / f64x8::splat(54.0) - t56 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t160 + f64x8::splat(4.0) / f64x8::splat(3.0) * t216 * t146) / f64x8::splat(8.0);
            let t478 = t16 * t477;
            let t482 = t76 * t10 * t83;
            let t485 = t73 * t182;
            let t497 = f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t113 * t223 + t117 * t118 * t448 / f64x8::splat(3.0) + t123 * t452 * t108 / f64x8::splat(3.0) + t13 * t478 + f64x8::splat(7.0) / f64x8::splat(108.0) * t169 * t173 * t482 + f64x8::splat(7.0) / f64x8::splat(108.0) * t485 * t10 * t83 * t172 * param_d + f64x8::splat(7.0) / f64x8::splat(54.0) * t77 * t192 * t195 - f64x8::splat(7.0) / f64x8::splat(12.0) * t77 * t10 * t201;
            let tv2rhosigma0 = param_a * t228 + t92 * t497;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
