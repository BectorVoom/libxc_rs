//! GGA_X_PBE vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 53 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_pbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_kappa: f64,
    param_mu: f64,
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
        // --- shared preamble (51 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = param_kappa + t29 * t34 * t39 / 24.0;
        let t48 = 1.0 + param_kappa * (1.0 - param_kappa / t43);
        let t52 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(t57);
        let t61 = piecewise3(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = t33 * sigma2;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t72 = param_kappa + t29 * t63 * t68 / 24.0;
        let t77 = 1.0 + param_kappa * (1.0 - param_kappa / t72);
        let t81 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t77);
        let tzk0 = t52 + t81;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (53 lines) ---
        let t82 = t6 * t6;
        let t83 = 1.0 / t82;
        let t84 = t16 * t83;
        let t86 = piecewise5(t10, 0.0, t14, 0.0, t7 - t84);
        let t89 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t86);
        let t90 = t89 * t26;
        let t94 = t26 * t26;
        let t95 = 1.0 / t94;
        let t96 = t25 * t95;
        let t99 = t5 * t96 * t48 / 8.0;
        let t100 = param_kappa * param_kappa;
        let t101 = t27 * t100;
        let t102 = t5 * t101;
        let t103 = t43 * t43;
        let t105 = 1.0 / t103 * param_mu;
        let t106 = t105 * t28;
        let t107 = t35 * rho0;
        let t109 = 1.0 / t37 / t107;
        let t111 = t106 * t34 * t109;
        let t115 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t90 * t48 - t99 + t102 * t111 / 24.0);
        let t116 = t54 * t83;
        let t118 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t116);
        let t121 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t118);
        let t122 = t121 * t26;
        let t126 = t61 * t95;
        let t129 = t5 * t126 * t77 / 8.0;
        let t131 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t122 * t77 - t129);
        let tvrho0 = t52 + t81 + t6 * (t115 + t131);
        vrho[ip * 2] += tvrho0;
        let t135 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t84);
        let t138 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t135);
        let t139 = t138 * t26;
        let t144 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t139 * t48 - t99);
        let t146 = piecewise5(t14, 0.0, t10, 0.0, t7 - t116);
        let t149 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t146);
        let t150 = t149 * t26;
        let t154 = t62 * t100;
        let t155 = t5 * t154;
        let t156 = t72 * t72;
        let t158 = 1.0 / t156 * param_mu;
        let t159 = t158 * t28;
        let t160 = t64 * rho1;
        let t162 = 1.0 / t66 / t160;
        let t164 = t159 * t63 * t162;
        let t168 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t150 * t77 - t129 + t155 * t164 / 24.0);
        let tvrho1 = t52 + t81 + t6 * (t144 + t168);
        vrho[ip * 2 + 1] += tvrho1;
        let t171 = t28 * t33;
        let t173 = t105 * t171 * t39;
        let t176 = piecewise3(t1, 0.0, -t102 * t173 / 64.0);
        let tvsigma0 = t6 * t176;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t178 = t158 * t171 * t68;
        let t181 = piecewise3(t53, 0.0, -t155 * t178 / 64.0);
        let tvsigma2 = t6 * t181;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
