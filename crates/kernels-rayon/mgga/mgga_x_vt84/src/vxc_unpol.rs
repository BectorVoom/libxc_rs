//! MGGA_X_VT84 vxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_vt84_vxc_unpol(
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
