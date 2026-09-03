//! MGGA_X_RTPSS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
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
pub fn mgga_x_rtpss_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t21 = v_sigma * v_sigma;
            let t22 = param_c * t21;
            let t23 = v_rho * v_rho;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = v_tau * v_tau;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t24 * t26;
            let t28 = t21 * t24;
            let t29 = t28 * t26;
            let t31 = f64x8::splat(1.0) + t29 / f64x8::splat(64.0);
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t27 * t33;
            let t38 = f64x8::splat(M_CBRT6);
            let t39 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t22 * t34 / f64x8::splat(64.0)) * t38;
            let t40 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t41 = (simd::cbrt(t40));
            let t42 = t41 * t41;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t43;
            let t45 = f64x8::splat(M_CBRT2);
            let t46 = t45 * t45;
            let t47 = v_sigma * t46;
            let t48 = t19 * t19;
            let t50 = f64x8::splat(1.0) / t48 / t23;
            let t51 = t47 * t50;
            let t54 = v_tau * t46;
            let t56 = f64x8::splat(1.0) / t48 / v_rho;
            let t59 = t54 * t56 - t51 / f64x8::splat(8.0);
            let t63 = f64x8::splat(5.0) / f64x8::splat(9.0) * t59 * t38 * t43 - f64x8::splat(1.0);
            let t64 = param_b * t59;
            let t65 = t38 * t43;
            let t66 = t65 * t63;
            let t69 = f64x8::splat(5.0) * t64 * t66 + f64x8::splat(9.0);
            let t70 = ((t69).sqrt());
            let t71 = f64x8::splat(1.0) / t70;
            let t76 = f64x8::splat(27.0) / f64x8::splat(20.0) * t63 * t71 + t65 * t51 / f64x8::splat(36.0);
            let t77 = t76 * t76;
            let t80 = t38 * t38;
            let t82 = f64x8::splat(1.0) / t41 / t40;
            let t83 = t80 * t82;
            let t84 = t21 * t45;
            let t85 = t23 * t23;
            let t86 = t85 * v_rho;
            let t88 = f64x8::splat(1.0) / t19 / t86;
            let t89 = t84 * t88;
            let t92 = f64x8::splat(100.0) * t83 * t89 + f64x8::splat(162.0) * t29;
            let t93 = ((t92).sqrt());
            let t96 = f64x8::splat(1.0) / param_kappa;
            let t97 = t96 * t80;
            let t98 = t97 * t82;
            let t101 = ((param_e).sqrt());
            let t102 = t101 * t21;
            let t105 = param_e * param_mu;
            let t106 = t40 * t40;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t21 * v_sigma;
            let t109 = t107 * t108;
            let t110 = t85 * t85;
            let t111 = f64x8::splat(1.0) / t110;
            let t115 = t44 * t51 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t77 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t76 * t93 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t98 * t89 + t102 * t27 / f64x8::splat(720.0) + t105 * t109 * t111 / f64x8::splat(576.0);
            let t116 = t101 * t38;
            let t120 = f64x8::splat(1.0) + t116 * t43 * t51 / f64x8::splat(24.0);
            let t121 = t120 * t120;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = (simd::exp(-t115 * t122 * t96));
            let t128 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t125);
            let t132 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t128));
            let tzk0 = f64x8::splat(2.0) * t132;
            acc_zk = tzk0;
            let t133 = f64x8::splat(1.0) / t48;
            let t134 = t18 * t133;
            let t138 = t7 * t18;
            let t139 = t19 * param_kappa;
            let t140 = t23 * v_rho;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t141 * t26;
            let t143 = t142 * t33;
            let t146 = t21 * t21;
            let t147 = param_c * t146;
            let t148 = f64x8::splat(1.0) / t86;
            let t149 = t25 * t25;
            let t150 = f64x8::splat(1.0) / t149;
            let t153 = f64x8::splat(1.0) / t32 / t31;
            let t154 = t148 * t150 * t153;
            let t158 = (-t22 * t143 / f64x8::splat(32.0) + t147 * t154 / f64x8::splat(1024.0)) * t38;
            let t159 = t158 * t43;
            let t163 = f64x8::splat(1.0) / t48 / t140;
            let t164 = t47 * t163;
            let t170 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t54 * t50 + t164 / f64x8::splat(3.0);
            let t171 = t170 * t38;
            let t172 = t43 * t71;
            let t176 = f64x8::splat(1.0) / t70 / t69;
            let t177 = t63 * t176;
            let t181 = t83 * t170;
            let t184 = f64x8::splat(5.0) * param_b * t170 * t66 + f64x8::splat(25.0) / f64x8::splat(9.0) * t64 * t181;
            let t187 = t65 * t164;
            let t189 = f64x8::splat(3.0) / f64x8::splat(4.0) * t171 * t172 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t184 - f64x8::splat(2.0) / f64x8::splat(27.0) * t187;
            let t194 = f64x8::splat(1.0) / t93;
            let t195 = t76 * t194;
            let t196 = t21 * t141;
            let t199 = t85 * t23;
            let t201 = f64x8::splat(1.0) / t19 / t199;
            let t202 = t84 * t201;
            let t205 = -f64x8::splat(324.0) * t196 * t26 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t83 * t202;
            let t212 = t110 * v_rho;
            let t213 = f64x8::splat(1.0) / t212;
            let t217 = t159 * t51 / f64x8::splat(24.0) - t44 * t164 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t189 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t189 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t205 - f64x8::splat(50.0) / f64x8::splat(177147.0) * t98 * t202 - t102 * t142 / f64x8::splat(360.0) - t105 * t109 * t213 / f64x8::splat(72.0);
            let t220 = t121 * t120;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = t96 * t101;
            let t224 = t115 * t221 * t223;
            let t227 = -t217 * t122 * t96 - f64x8::splat(2.0) / f64x8::splat(9.0) * t224 * t187;
            let t228 = t227 * t125;
            let t233 = ((t3).select(f64x8::splat(0.0), -t7 * t134 * t128 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t228));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t233 + f64x8::splat(2.0) * t132;
            acc_vrho = tvrho0;
            let t236 = param_c * v_sigma;
            let t239 = param_c * t108;
            let t240 = f64x8::splat(1.0) / t85;
            let t241 = t240 * t150;
            let t242 = t241 * t153;
            let t246 = (t236 * t34 / f64x8::splat(32.0) - t239 * t242 / f64x8::splat(1024.0)) * t38;
            let t247 = t246 * t43;
            let t250 = t43 * t46;
            let t251 = t250 * t50;
            let t254 = t46 * t50;
            let t255 = t65 * t71;
            let t256 = t254 * t255;
            let t258 = param_b * t46;
            let t259 = t258 * t50;
            let t260 = t259 * t66;
            let t262 = t64 * t80;
            let t263 = t82 * t46;
            let t265 = t262 * t263 * t50;
            let t267 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t260 - f64x8::splat(25.0) / f64x8::splat(72.0) * t265;
            let t270 = t254 * t65;
            let t272 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t256 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t267 + t270 / f64x8::splat(36.0);
            let t277 = v_sigma * t24;
            let t280 = v_sigma * t45;
            let t281 = t280 * t88;
            let t284 = f64x8::splat(324.0) * t277 * t26 + f64x8::splat(200.0) * t83 * t281;
            let t289 = t101 * v_sigma;
            let t292 = t107 * t21;
            let t296 = t247 * t51 / f64x8::splat(24.0) + t39 * t251 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t272 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t272 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t284 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t98 * t281 + t289 * t27 / f64x8::splat(360.0) + t105 * t292 * t111 / f64x8::splat(192.0);
            let t301 = -t296 * t122 * t96 + t224 * t270 / f64x8::splat(12.0);
            let t302 = t301 * t125;
            let t306 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t302));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t306;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t308 = t25 * v_tau;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t24 * t309;
            let t311 = t310 * t33;
            let t314 = t149 * v_tau;
            let t315 = f64x8::splat(1.0) / t314;
            let t317 = t240 * t315 * t153;
            let t321 = (-t22 * t311 / f64x8::splat(32.0) + t147 * t317 / f64x8::splat(1024.0)) * t38;
            let t322 = t321 * t43;
            let t325 = t46 * t56;
            let t328 = t258 * t56;
            let t334 = f64x8::splat(5.0) * t328 * t66 + f64x8::splat(25.0) / f64x8::splat(9.0) * t262 * t263 * t56;
            let t337 = f64x8::splat(3.0) / f64x8::splat(4.0) * t325 * t255 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t334;
            let t342 = t28 * t309;
            let t347 = t322 * t51 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t337 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t337 * t93 + f64x8::splat(73.0) / f64x8::splat(600.0) * t195 * t342 - t102 * t310 / f64x8::splat(360.0);
            let t349 = t122 * t125;
            let t353 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t19 * t347 * t349));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t353;
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
