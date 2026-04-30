//! MGGA_K_CSK vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 34 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_csk_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_csk_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (51 lines) ---
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
        let t30 = t25 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t38 = t30 * t33 * t36;
        let t40 = lapl[ip] * t32;
        let t42 = 1.0 / t23 / rho[ip];
        let t47 = 5.0 / 54.0 * t30 * t40 * t42 - 5.0 / 81.0 * t38;
        let t49 = f64::ln(1.0 - f64::EPSILON);
        let t50 = 1.0 / param_csk_a;
        let t51 = f64::powf(-t49, -t50);
        let t52 = t47 < -t51;
        let t53 = f64::ln(f64::EPSILON);
        let t54 = f64::powf(-t53, -t50);
        let t55 = -t54 < t47;
        let t56 = piecewise3(t55, -t54, t47);
        let t57 = -t51 < t56;
        let t58 = piecewise3(t57, t56, -t51);
        let t59 = f64::abs(t58);
        let t60 = f64::powf(t59, param_csk_a);
        let t61 = 1.0 / t60;
        let t62 = f64::exp(-t61);
        let t63 = 1.0 - t62;
        let t64 = f64::powf(t63, t50);
        let t65 = piecewise5(t52, 0.0, t55, 1.0, t64);
        let t67 = 1.0 + 5.0 / 72.0 * t38 + t47 * t65;
        let t71 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (34 lines) ---
        let t73 = t21 / t22;
        let t77 = t34 * rho[ip];
        let t79 = 1.0 / t23 / t77;
        let t81 = t30 * t33 * t79;
        let t87 = -25.0 / 162.0 * t30 * t40 * t36 + 40.0 / 243.0 * t81;
        let t89 = t64 * t61;
        let t90 = piecewise3(t55, 0.0, t87);
        let t91 = piecewise3(t57, t90, 0.0);
        let t93 = f64::abs(t58) / t58;
        let t94 = 1.0 / t59;
        let t96 = 1.0 / t63;
        let t97 = t62 * t96;
        let t98 = t93 * t94 * t97;
        let t100 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t91 * t98);
        let t102 = -5.0 / 27.0 * t81 + t87 * t65 + t47 * t100;
        let t107 = piecewise3(t3, 0.0, t8 * t73 * t67 / 10.0 + 3.0 / 20.0 * t8 * t24 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t110 = t32 * t36;
        let t111 = t30 * t110;
        let t114 = t30 * t110 * t65;
        let t117 = piecewise3(t55, 0.0, -5.0 / 81.0 * t111);
        let t118 = piecewise3(t57, t117, 0.0);
        let t121 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t118 * t98);
        let t123 = 5.0 / 72.0 * t111 - 5.0 / 81.0 * t114 + t47 * t121;
        let t127 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        let t129 = t32 * t42;
        let t135 = piecewise3(t55, 0.0, 5.0 / 54.0 * t30 * t129);
        let t136 = piecewise3(t57, t135, 0.0);
        let t139 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t136 * t98);
        let t141 = 5.0 / 54.0 * t30 * t129 * t65 + t47 * t139;
        let t145 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t141);
        let tvlapl0 = 2.0 * rho[ip] * t145;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
