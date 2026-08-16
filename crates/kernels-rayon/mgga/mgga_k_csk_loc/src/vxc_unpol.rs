//! MGGA_K_CSK_LOC vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_csk_loc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_csk_loc_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_csk_a: f64,
    param_csk_cp: f64,
    param_csk_cq: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t39 = 5.0 / 72.0 * t30 * t37;
        let t40 = param_csk_cp * t25;
        let t41 = t40 * t29;
        let t44 = param_csk_cq * t25;
        let t45 = t44 * t29;
        let t46 = lapl[ip] * t32;
        let t48 = 1.0 / t23 / rho[ip];
        let t52 = t41 * t37 / 24.0 + t45 * t46 * t48 / 24.0 - t39;
        let t54 = f64::ln(1.0 - f64::EPSILON);
        let t55 = 1.0 / param_csk_a;
        let t56 = f64::powf(-t54, -t55);
        let t57 = t52 < -t56;
        let t58 = f64::ln(f64::EPSILON);
        let t59 = f64::powf(-t58, -t55);
        let t60 = -t59 < t52;
        let t61 = piecewise3(t60, -t59, t52);
        let t62 = -t56 < t61;
        let t63 = piecewise3(t62, t61, -t56);
        let t64 = f64::abs(t63);
        let t65 = f64::powf(t64, param_csk_a);
        let t66 = 1.0 / t65;
        let t67 = f64::exp(-t66);
        let t68 = 1.0 - t67;
        let t69 = f64::powf(t68, t55);
        let t70 = piecewise5(t57, 0.0, t60, 1.0, t69);
        let t72 = t52 * t70 + t39 + 1.0;
        let t76 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
        let t78 = t21 / t22;
        let t82 = t34 * rho[ip];
        let t84 = 1.0 / t23 / t82;
        let t85 = t33 * t84;
        let t87 = 5.0 / 27.0 * t30 * t85;
        let t93 = -t41 * t85 / 9.0 - 5.0 / 72.0 * t45 * t46 * t36 + t87;
        let t95 = t69 * t66;
        let t96 = piecewise3(t60, 0.0, t93);
        let t97 = piecewise3(t62, t96, 0.0);
        let t99 = f64::abs(t63) / t63;
        let t100 = 1.0 / t64;
        let t102 = 1.0 / t68;
        let t103 = t67 * t102;
        let t104 = t99 * t100 * t103;
        let t106 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t97 * t104);
        let t108 = t52 * t106 + t93 * t70 - t87;
        let t113 = piecewise3(t3, 0.0, t8 * t78 * t72 / 10.0 + 3.0 / 20.0 * t8 * t24 * t108);
        let tvrho0 = 2.0 * rho[ip] * t113 + 2.0 * t76;
        vrho[ip] += tvrho0;
        let t116 = t32 * t36;
        let t118 = 5.0 / 72.0 * t30 * t116;
        let t119 = t29 * t32;
        let t120 = t119 * t36;
        let t123 = t40 * t120 / 24.0 - t118;
        let t125 = piecewise3(t60, 0.0, t123);
        let t126 = piecewise3(t62, t125, 0.0);
        let t129 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t126 * t104);
        let t131 = t123 * t70 + t52 * t129 + t118;
        let t135 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t131);
        let tvsigma0 = 2.0 * rho[ip] * t135;
        vsigma[ip] += tvsigma0;
        let t137 = t32 * t48;
        let t144 = piecewise3(t60, 0.0, t44 * t119 * t48 / 24.0);
        let t145 = piecewise3(t62, t144, 0.0);
        let t148 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t145 * t104);
        let t150 = t45 * t137 * t70 / 24.0 + t52 * t148;
        let t154 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t150);
        let tvlapl0 = 2.0 * rho[ip] * t154;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
