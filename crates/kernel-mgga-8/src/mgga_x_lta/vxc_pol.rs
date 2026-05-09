//! MGGA_X_LTA vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 43 shared lines across all orders.
//! Delta: 53 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_lta_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (43 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t34 = M_CBRT6;
        let t35 = M_PI * M_PI;
        let t36 = pow_1_3(t35);
        let t37 = t36 * t36;
        let t39 = t34 / t37;
        let t42 = 4.0 / 5.0 * param_ltafrac;
        let t43 = f64::powf(5.0 / 9.0 * tau0 / t30 / rho0 * t39, t42);
        let t47 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t17;
        let t51 = piecewise5(t15, t12, t11, t16, t49 * t8);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t23, t54 * t52);
        let t57 = t56 * t27;
        let t58 = pow_1_3(rho1);
        let t59 = t58 * t58;
        let t65 = f64::powf(5.0 / 9.0 * tau1 / t59 / rho1 * t39, t42);
        let t69 = piecewise3(t48, 0.0, -3.0 / 8.0 * t6 * t57 * t65);
        let tzk0 = t47 + t69;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (53 lines) ---
        let t70 = t7 * t7;
        let t71 = 1.0 / t70;
        let t72 = t17 * t71;
        let t74 = piecewise5(t11, 0.0, t15, 0.0, t8 - t72);
        let t77 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t74);
        let t78 = t77 * t27;
        let t82 = t27 * t27;
        let t83 = 1.0 / t82;
        let t84 = t26 * t83;
        let t87 = t6 * t84 * t43 / 8.0;
        let t88 = t6 * t26;
        let t89 = t27 * t43;
        let t90 = 1.0 / rho0;
        let t91 = param_ltafrac * t90;
        let t92 = t89 * t91;
        let t96 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t78 * t43 - t87 + t88 * t92 / 2.0);
        let t97 = t49 * t71;
        let t99 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t97);
        let t102 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t99);
        let t103 = t102 * t27;
        let t107 = t56 * t83;
        let t110 = t6 * t107 * t65 / 8.0;
        let t112 = piecewise3(t48, 0.0, -3.0 / 8.0 * t6 * t103 * t65 - t110);
        let tvrho0 = t47 + t69 + t7 * (t96 + t112);
        vrho[ip * 2] += tvrho0;
        let t116 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t72);
        let t119 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t116);
        let t120 = t119 * t27;
        let t125 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t120 * t43 - t87);
        let t127 = piecewise5(t15, 0.0, t11, 0.0, t8 - t97);
        let t130 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t127);
        let t131 = t130 * t27;
        let t135 = t6 * t56;
        let t136 = t27 * t65;
        let t137 = 1.0 / rho1;
        let t138 = param_ltafrac * t137;
        let t139 = t136 * t138;
        let t143 = piecewise3(t48, 0.0, -3.0 / 8.0 * t6 * t131 * t65 - t110 + t135 * t139 / 2.0);
        let tvrho1 = t47 + t69 + t7 * (t125 + t143);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t146 = 1.0 / tau0;
        let t147 = param_ltafrac * t146;
        let t148 = t89 * t147;
        let t151 = piecewise3(t2, 0.0, -3.0 / 10.0 * t88 * t148);
        let tvtau0 = t7 * t151;
        vtau[ip * 2] += tvtau0;
        let t152 = 1.0 / tau1;
        let t153 = param_ltafrac * t152;
        let t154 = t136 * t153;
        let t157 = piecewise3(t48, 0.0, -3.0 / 10.0 * t135 * t154);
        let tvtau1 = t7 * t157;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
