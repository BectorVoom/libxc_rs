//! MGGA_X_TPSS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
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
pub fn mgga_x_tpss_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_BLOC_a = f64x8::splat(param_BLOC_a);
    let param_BLOC_b = f64x8::splat(param_BLOC_b);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = v_sigma * t21 * t23 / f64x8::splat(8.0);
            let t26 = param_BLOC_b * v_sigma;
            let t30 = param_BLOC_a + t26 * t21 * t23 / f64x8::splat(8.0);
            let t31 = (simd::pow(t25, t30));
            let t32 = param_c * t31;
            let t33 = v_sigma * v_sigma;
            let t34 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t33 * t35;
            let t37 = v_tau * v_tau;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t36 * t38;
            let t41 = f64x8::splat(1.0) + t39 / f64x8::splat(64.0);
            let t42 = t41 * t41;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t32 * t43) * t46;
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t47 * t51;
            let t53 = f64x8::splat(M_CBRT2);
            let t54 = t53 * t53;
            let t55 = v_sigma * t54;
            let t56 = t19 * t19;
            let t58 = f64x8::splat(1.0) / t56 / t34;
            let t59 = t55 * t58;
            let t62 = v_tau * t54;
            let t64 = f64x8::splat(1.0) / t56 / v_rho;
            let t67 = t62 * t64 - t59 / f64x8::splat(8.0);
            let t71 = f64x8::splat(5.0) / f64x8::splat(9.0) * t67 * t46 * t51 - f64x8::splat(1.0);
            let t72 = param_b * t67;
            let t73 = t46 * t51;
            let t74 = t73 * t71;
            let t77 = f64x8::splat(5.0) * t72 * t74 + f64x8::splat(9.0);
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t84 = f64x8::splat(27.0) / f64x8::splat(20.0) * t71 * t79 + t73 * t59 / f64x8::splat(36.0);
            let t85 = t84 * t84;
            let t88 = t46 * t46;
            let t90 = f64x8::splat(1.0) / t49 / t48;
            let t91 = t88 * t90;
            let t92 = t33 * t53;
            let t93 = t34 * t34;
            let t94 = t93 * v_rho;
            let t96 = f64x8::splat(1.0) / t19 / t94;
            let t97 = t92 * t96;
            let t100 = f64x8::splat(100.0) * t91 * t97 + f64x8::splat(162.0) * t39;
            let t101 = ((t100).sqrt());
            let t105 = f64x8::splat(1.0) / param_kappa * t88;
            let t106 = t105 * t90;
            let t109 = ((param_e).sqrt());
            let t110 = t109 * t33;
            let t111 = t35 * t38;
            let t114 = param_e * param_mu;
            let t115 = t48 * t48;
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t33 * v_sigma;
            let t118 = t116 * t117;
            let t119 = t93 * t93;
            let t120 = f64x8::splat(1.0) / t119;
            let t124 = t52 * t59 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t85 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t84 * t101 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t106 * t97 + t110 * t111 / f64x8::splat(720.0) + t114 * t118 * t120 / f64x8::splat(576.0);
            let t125 = t109 * t46;
            let t129 = f64x8::splat(1.0) + t125 * t51 * t59 / f64x8::splat(24.0);
            let t130 = t129 * t129;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t124 * t131 + param_kappa;
            let t138 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t133);
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t56;
            let t144 = t18 * t143;
            let t148 = t7 * t18;
            let t149 = param_kappa * param_kappa;
            let t150 = t19 * t149;
            let t151 = t133 * t133;
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t35 * t23;
            let t154 = (simd::ln(t25));
            let t159 = -t26 * t153 * t154 / f64x8::splat(8.0) - t30 * t21;
            let t160 = t159 * t43;
            let t163 = f64x8::splat(1.0) / t42 / t41;
            let t164 = t32 * t163;
            let t165 = t34 * v_rho;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t33 * t166;
            let t168 = t167 * t38;
            let t172 = (t32 * t160 + t164 * t168 / f64x8::splat(16.0)) * t46;
            let t173 = t172 * t51;
            let t177 = f64x8::splat(1.0) / t56 / t165;
            let t178 = t55 * t177;
            let t184 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t58 + t178 / f64x8::splat(3.0);
            let t185 = t184 * t46;
            let t186 = t51 * t79;
            let t190 = f64x8::splat(1.0) / t78 / t77;
            let t191 = t71 * t190;
            let t195 = t91 * t184;
            let t198 = f64x8::splat(5.0) * param_b * t184 * t74 + f64x8::splat(25.0) / f64x8::splat(9.0) * t72 * t195;
            let t203 = f64x8::splat(3.0) / f64x8::splat(4.0) * t185 * t186 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t198 - f64x8::splat(2.0) / f64x8::splat(27.0) * t73 * t178;
            let t208 = f64x8::splat(1.0) / t101;
            let t209 = t84 * t208;
            let t211 = t93 * t34;
            let t213 = f64x8::splat(1.0) / t19 / t211;
            let t214 = t92 * t213;
            let t217 = -f64x8::splat(324.0) * t168 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t91 * t214;
            let t222 = t166 * t38;
            let t225 = t119 * v_rho;
            let t226 = f64x8::splat(1.0) / t225;
            let t230 = t173 * t59 / f64x8::splat(24.0) - t52 * t178 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t203 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t203 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t217 - f64x8::splat(50.0) / f64x8::splat(177147.0) * t106 * t214 - t110 * t222 / f64x8::splat(360.0) - t114 * t118 * t226 / f64x8::splat(72.0);
            let t232 = t130 * t129;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t124 * t233;
            let t235 = t234 * t125;
            let t236 = t51 * v_sigma;
            let t237 = t54 * t177;
            let t238 = t236 * t237;
            let t241 = t230 * t131 + f64x8::splat(2.0) / f64x8::splat(9.0) * t235 * t238;
            let t242 = t152 * t241;
            let t247 = ((t3).select(f64x8::splat(0.0), -t7 * t144 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t242));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t247 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t250 = param_BLOC_b * t21;
            let t251 = t23 * t154;
            let t254 = f64x8::splat(1.0) / v_sigma;
            let t256 = t250 * t251 / f64x8::splat(8.0) + t30 * t254;
            let t257 = t256 * t43;
            let t258 = t32 * t257;
            let t259 = v_sigma * t35;
            let t260 = t259 * t38;
            let t264 = (t258 - t164 * t260 / f64x8::splat(16.0)) * t46;
            let t265 = t264 * t51;
            let t268 = t51 * t54;
            let t269 = t268 * t58;
            let t272 = t54 * t58;
            let t273 = t73 * t79;
            let t274 = t272 * t273;
            let t276 = param_b * t54;
            let t277 = t276 * t58;
            let t278 = t277 * t74;
            let t280 = t72 * t88;
            let t281 = t90 * t54;
            let t283 = t280 * t281 * t58;
            let t285 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t278 - f64x8::splat(25.0) / f64x8::splat(72.0) * t283;
            let t288 = t272 * t73;
            let t290 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t274 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t285 + t288 / f64x8::splat(36.0);
            let t296 = v_sigma * t53;
            let t297 = t296 * t96;
            let t300 = f64x8::splat(200.0) * t91 * t297 + f64x8::splat(324.0) * t260;
            let t305 = t109 * v_sigma;
            let t308 = t116 * t33;
            let t312 = t265 * t59 / f64x8::splat(24.0) + t47 * t269 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t290 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t290 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t300 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t106 * t297 + t305 * t111 / f64x8::splat(360.0) + t114 * t308 * t120 / f64x8::splat(192.0);
            let t314 = t234 * t109;
            let t317 = t312 * t131 - t314 * t288 / f64x8::splat(12.0);
            let t318 = t152 * t317;
            let t322 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t318));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t322;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t324 = t7 * t20;
            let t325 = t149 * t152;
            let t326 = t21 * t38;
            let t331 = -t26 * t326 * t154 / f64x8::splat(8.0) - t30 * t23;
            let t332 = t331 * t43;
            let t333 = t32 * t332;
            let t334 = t37 * v_tau;
            let t335 = f64x8::splat(1.0) / t334;
            let t336 = t36 * t335;
            let t340 = (t333 + t164 * t336 / f64x8::splat(16.0)) * t46;
            let t341 = t340 * t51;
            let t344 = t54 * t64;
            let t347 = t276 * t64;
            let t353 = f64x8::splat(5.0) * t347 * t74 + f64x8::splat(25.0) / f64x8::splat(9.0) * t280 * t281 * t64;
            let t356 = f64x8::splat(3.0) / f64x8::splat(4.0) * t344 * t273 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t353;
            let t363 = t35 * t335;
            let t366 = t341 * t59 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t356 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t356 * t101 + f64x8::splat(73.0) / f64x8::splat(600.0) * t209 * t336 - t110 * t363 / f64x8::splat(360.0);
            let t367 = t366 * t131;
            let t368 = t325 * t367;
            let t371 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t324 * t368));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t371;
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
