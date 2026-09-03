//! MGGA_X_REGTPSS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`
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
pub fn mgga_x_regtpss_vxc_unpol(
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
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t24 = t22 * t23;
            let t25 = ((t24) * (t24) * (t24));
            let t26 = v_sigma * v_sigma;
            let t27 = v_rho * v_rho;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t26 * t28;
            let t30 = v_tau * v_tau;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t29 * t31;
            let t34 = f64x8::splat(1.0) + t32 / f64x8::splat(64.0);
            let t35 = t34 * t34;
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t25 * t36) * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t41 * t45;
            let t47 = f64x8::splat(M_CBRT2);
            let t48 = t47 * t47;
            let t49 = v_sigma * t48;
            let t50 = t19 * t19;
            let t52 = f64x8::splat(1.0) / t50 / t27;
            let t53 = t49 * t52;
            let t56 = v_tau * t48;
            let t58 = f64x8::splat(1.0) / t50 / v_rho;
            let t61 = t56 * t58 - t53 / f64x8::splat(8.0);
            let t62 = t61 * t40;
            let t63 = t62 * t45;
            let t65 = f64x8::splat(5.0) / f64x8::splat(9.0) * t63 - f64x8::splat(1.0);
            let t66 = t45 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t62 * t66;
            let t70 = ((t69).sqrt());
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = t40 * t45;
            let t75 = t74 * t53;
            let t76 = t75 / f64x8::splat(36.0);
            let t77 = f64x8::splat(9.0) / f64x8::splat(20.0) * t65 * t71 + t76;
            let t78 = t77 * t77;
            let t81 = t40 * t40;
            let t83 = f64x8::splat(1.0) / t43 / t42;
            let t84 = t81 * t83;
            let t85 = t26 * t47;
            let t86 = t27 * t27;
            let t87 = t86 * v_rho;
            let t89 = f64x8::splat(1.0) / t19 / t87;
            let t91 = t84 * t85 * t89;
            let t93 = f64x8::splat(162.0) * t32 + f64x8::splat(100.0) * t91;
            let t94 = ((t93).sqrt());
            let t97 = f64x8::splat(6.582356890714508e-05) * t91;
            let t99 = t26 * v_sigma;
            let t100 = t86 * t86;
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = f64x8::splat(5.408850610708026e-06) * t99 * t101;
            let t104 = t46 * t53 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t78 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t77 * t94 + t97 + f64x8::splat(0.0020448759451792767) * t32 + t103;
            let t106 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t75;
            let t107 = t106 * t106;
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = f64x8::splat(0.804) + t104 * t108;
            let t112 = f64x8::splat(0.646416) / t110;
            let t113 = -t65;
            let t114 = t113 * t113;
            let t115 = t114 * t113;
            let t116 = t61 * t61;
            let t117 = t116 * t81;
            let t118 = t117 * t83;
            let t120 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t118;
            let t121 = ((t120).sqrt());
            let t123 = f64x8::splat(1.0) / t121 / t120;
            let t124 = t115 * t123;
            let t126 = (simd::exp(-t75 / f64x8::splat(8.0)));
            let t128 = -f64x8::splat(0.45) + t76;
            let t129 = t128 * t128;
            let t132 = f64x8::splat(2592.0) + f64x8::splat(25.0) * t91;
            let t133 = ((t132).sqrt());
            let t136 = f64x8::splat(0.029644443963477367) * t75 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t129 - f64x8::splat(73.0) / f64x8::splat(48600.0) * t128 * t133 + t97 + f64x8::splat(0.1308720604914737) + t103;
            let t138 = f64x8::splat(0.804) + t136 * t108;
            let t141 = -f64x8::splat(0.646416) / t138 + t112;
            let t142 = t126 * t141;
            let t144 = f64x8::splat(1.804) - t112 + t124 * t142;
            let t148 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t144));
            let tzk0 = f64x8::splat(2.0) * t148;
            acc_zk = tzk0;
            let t150 = t18 / t50;
            let t154 = t110 * t110;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = ((t24) * (t24));
            let t157 = t156 * t36;
            let t158 = v_sigma * t28;
            let t163 = f64x8::splat(1.0) / t35 / t34;
            let t164 = t25 * t163;
            let t165 = t27 * v_rho;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t26 * t166;
            let t168 = t167 * t31;
            let t172 = (-f64x8::splat(0.0137814812109375) * t157 * t158 * t23 + f64x8::splat(0.00028711419189453123) * t164 * t168) * t40;
            let t173 = t172 * t45;
            let t177 = f64x8::splat(1.0) / t50 / t165;
            let t178 = t49 * t177;
            let t184 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t56 * t52 + t178 / f64x8::splat(3.0);
            let t185 = t184 * t40;
            let t186 = t45 * t71;
            let t190 = f64x8::splat(1.0) / t70 / t69;
            let t191 = t65 * t190;
            let t194 = t61 * t81;
            let t195 = t83 * t184;
            let t196 = t194 * t195;
            let t198 = f64x8::splat(0.2222222222222222) * t185 * t66 + f64x8::splat(0.12345679012345678) * t196;
            let t201 = t74 * t178;
            let t203 = t185 * t186 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t191 * t198 - f64x8::splat(2.0) / f64x8::splat(27.0) * t201;
            let t208 = f64x8::splat(1.0) / t94;
            let t209 = t77 * t208;
            let t211 = t86 * t27;
            let t213 = f64x8::splat(1.0) / t19 / t211;
            let t215 = t84 * t85 * t213;
            let t217 = -f64x8::splat(324.0) * t168 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t215;
            let t220 = f64x8::splat(0.00035105903417144045) * t215;
            let t222 = t100 * v_rho;
            let t223 = f64x8::splat(1.0) / t222;
            let t225 = f64x8::splat(4.3270804885664206e-05) * t99 * t223;
            let t226 = t173 * t53 / f64x8::splat(24.0) - t46 * t178 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t77 * t203 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t203 * t94 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t217 - t220 - f64x8::splat(0.004089751890358553) * t168 - t225;
            let t228 = t107 * t106;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t104 * t229;
            let t231 = t230 * t40;
            let t232 = t45 * v_sigma;
            let t233 = t48 * t177;
            let t234 = t232 * t233;
            let t237 = t226 * t108 + f64x8::splat(0.32718015122868427) * t231 * t234;
            let t239 = f64x8::splat(0.646416) * t155 * t237;
            let t240 = t114 * t123;
            let t241 = t240 * t126;
            let t242 = t141 * t184;
            let t246 = t120 * t120;
            let t248 = f64x8::splat(1.0) / t121 / t246;
            let t249 = t115 * t248;
            let t250 = t249 * t142;
            let t253 = t124 * t74;
            let t254 = t177 * t126;
            let t255 = t254 * t141;
            let t256 = t49 * t255;
            let t259 = t138 * t138;
            let t260 = f64x8::splat(1.0) / t259;
            let t263 = t128 * t40 * t45;
            let t266 = t74 * v_sigma;
            let t270 = f64x8::splat(1.0) / t133;
            let t272 = t128 * t270 * t81;
            let t273 = t83 * t26;
            let t274 = t47 * t213;
            let t278 = -f64x8::splat(0.07905185056927298) * t201 - f64x8::splat(584.0) / f64x8::splat(54675.0) * t263 * t178 + f64x8::splat(73.0) / f64x8::splat(656100.0) * t266 * t233 * t133 + f64x8::splat(73.0) / f64x8::splat(729.0) * t272 * t273 * t274 - t220 - t225;
            let t280 = t136 * t229;
            let t281 = t280 * t40;
            let t284 = t278 * t108 + f64x8::splat(0.32718015122868427) * t281 * t234;
            let t287 = f64x8::splat(0.646416) * t260 * t284 - t239;
            let t288 = t126 * t287;
            let t290 = t239 - f64x8::splat(5.0) / f64x8::splat(3.0) * t241 * t242 * t74 - f64x8::splat(2.0144675925925926) * t250 * t196 + t253 * t256 / f64x8::splat(3.0) + t124 * t288;
            let t295 = ((t3).select(f64x8::splat(0.0), -t7 * t150 * t144 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t290));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t295 + f64x8::splat(2.0) * t148;
            acc_vrho = tvrho0;
            let t301 = t158 * t31;
            let t305 = (f64x8::splat(0.0137814812109375) * t157 * t21 * t23 - f64x8::splat(0.00028711419189453123) * t164 * t301) * t40;
            let t306 = t305 * t45;
            let t309 = t45 * t48;
            let t310 = t309 * t52;
            let t313 = t48 * t52;
            let t314 = t74 * t71;
            let t315 = t313 * t314;
            let t317 = t74 * t65;
            let t318 = t313 * t317;
            let t320 = t83 * t48;
            let t321 = t320 * t52;
            let t322 = t194 * t321;
            let t324 = -f64x8::splat(0.027777777777777776) * t318 - f64x8::splat(0.015432098765432098) * t322;
            let t327 = t313 * t74;
            let t329 = -t315 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t191 * t324 + t327 / f64x8::splat(36.0);
            let t335 = v_sigma * t47;
            let t336 = t335 * t89;
            let t337 = t84 * t336;
            let t339 = f64x8::splat(324.0) * t301 + f64x8::splat(200.0) * t337;
            let t342 = f64x8::splat(0.00013164713781429015) * t337;
            let t345 = f64x8::splat(1.6226551832124077e-05) * t26 * t101;
            let t346 = t306 * t53 / f64x8::splat(24.0) + t41 * t310 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t77 * t329 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t329 * t94 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t339 + t342 + f64x8::splat(0.004089751890358553) * t301 + t345;
            let t348 = t230 * t48;
            let t349 = t52 * t40;
            let t350 = t349 * t45;
            let t353 = t346 * t108 - f64x8::splat(0.12269255671075659) * t348 * t350;
            let t355 = f64x8::splat(0.646416) * t155 * t353;
            let t356 = t240 * t142;
            let t357 = t356 * t327;
            let t359 = t250 * t322;
            let t361 = t124 * t313;
            let t362 = t74 * t142;
            let t366 = t128 * t48;
            let t369 = t74 * t133;
            let t372 = t83 * v_sigma;
            let t373 = t47 * t89;
            let t377 = f64x8::splat(0.029644443963477367) * t327 + f64x8::splat(73.0) / f64x8::splat(18225.0) * t366 * t350 - f64x8::splat(73.0) / f64x8::splat(1749600.0) * t313 * t369 - f64x8::splat(73.0) / f64x8::splat(1944.0) * t272 * t372 * t373 + t342 + t345;
            let t379 = t280 * t48;
            let t382 = t377 * t108 - f64x8::splat(0.12269255671075659) * t379 * t350;
            let t385 = f64x8::splat(0.646416) * t260 * t382 - t355;
            let t386 = t126 * t385;
            let t388 = t355 + f64x8::splat(5.0) / f64x8::splat(24.0) * t357 + f64x8::splat(0.25180844907407407) * t359 - t361 * t362 / f64x8::splat(8.0) + t124 * t386;
            let t392 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t388));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t392;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t397 = t30 * v_tau;
            let t398 = f64x8::splat(1.0) / t397;
            let t399 = t29 * t398;
            let t403 = (-f64x8::splat(0.0137814812109375) * t157 * t22 * t31 + f64x8::splat(0.00028711419189453123) * t164 * t399) * t40;
            let t404 = t403 * t45;
            let t407 = t48 * t58;
            let t412 = t320 * t58;
            let t413 = t194 * t412;
            let t415 = f64x8::splat(0.2222222222222222) * t407 * t317 + f64x8::splat(0.12345679012345678) * t413;
            let t418 = t407 * t314 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t191 * t415;
            let t426 = t404 * t53 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t77 * t418 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t418 * t94 + f64x8::splat(73.0) / f64x8::splat(600.0) * t209 * t399 - f64x8::splat(0.004089751890358553) * t399;
            let t427 = t155 * t426;
            let t428 = t427 * t108;
            let t430 = t407 * t74;
            let t435 = t124 * t126;
            let t438 = f64x8::splat(0.646416) * t428 - f64x8::splat(5.0) / f64x8::splat(3.0) * t356 * t430 - f64x8::splat(2.0144675925925926) * t250 * t413 - f64x8::splat(0.646416) * t435 * t428;
            let t442 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t438));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t442;
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
