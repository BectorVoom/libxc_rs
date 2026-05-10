//! GGA_X_LG93 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 97 shared lines across all orders.
//! Delta: 97 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lg93_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (97 lines) ---
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = t28 * t28;
        let t44 = 1.0 / t30 / t29;
        let t45 = t42 * t44;
        let t46 = sigma0 * sigma0;
        let t47 = t34 * t34;
        let t48 = t47 * rho0;
        let t50 = 1.0 / t35 / t48;
        let t54 = t46 * sigma0;
        let t55 = t47 * t47;
        let t56 = 1.0 / t55;
        let t59 = t29 * t29;
        let t62 = t28 / t31 / t59;
        let t63 = t46 * t46;
        let t64 = t55 * t34;
        let t66 = 1.0 / t36 / t64;
        let t73 = t42 / t30 / t59 / t29;
        let t74 = t63 * sigma0;
        let t75 = t55 * t48;
        let t77 = 1.0 / t35 / t75;
        let t81 = t63 * t46;
        let t82 = t55 * t55;
        let t83 = 1.0 / t82;
        let t86 = 1.0 + 0.20588079936467259283e0 * t40 + 0.51718749999999999998e-1 * t45 * t46 * t50 + 0.99883908074331051182e-4 * t54 * t56 + 0.21916594328703703703e-3 * t62 * t63 * t66 + 0.11831024546682098765e-2 * t73 * t74 * t77 + 0.11106816177675317211e-8 * t81 * t83;
        let t87 = f64::powf(t86, 0.24974e-1);
        let t88 = t27 * t87;
        let t90 = 1.0 + 0.41666666666666666666e-9 * t40;
        let t91 = 1.0 / t90;
        let t92 = t88 * t91;
        let t95 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t92);
        let t96 = rho1 <= dens_threshold;
        let t97 = -t16;
        let t99 = piecewise5(t14, t11, t10, t15, t97 * t7);
        let t100 = 1.0 + t99;
        let t101 = t100 <= zeta_threshold;
        let t102 = pow_1_3(t100);
        let t104 = piecewise3(t101, t22, t102 * t100);
        let t105 = t5 * t104;
        let t106 = rho1 * rho1;
        let t107 = pow_1_3(rho1);
        let t108 = t107 * t107;
        let t110 = 1.0 / t108 / t106;
        let t112 = t33 * sigma2 * t110;
        let t114 = sigma2 * sigma2;
        let t115 = t106 * t106;
        let t116 = t115 * rho1;
        let t118 = 1.0 / t107 / t116;
        let t122 = t114 * sigma2;
        let t123 = t115 * t115;
        let t124 = 1.0 / t123;
        let t127 = t114 * t114;
        let t128 = t123 * t106;
        let t130 = 1.0 / t108 / t128;
        let t134 = t127 * sigma2;
        let t135 = t123 * t116;
        let t137 = 1.0 / t107 / t135;
        let t141 = t127 * t114;
        let t142 = t123 * t123;
        let t143 = 1.0 / t142;
        let t146 = 1.0 + 0.20588079936467259283e0 * t112 + 0.51718749999999999998e-1 * t45 * t114 * t118 + 0.99883908074331051182e-4 * t122 * t124 + 0.21916594328703703703e-3 * t62 * t127 * t130 + 0.11831024546682098765e-2 * t73 * t134 * t137 + 0.11106816177675317211e-8 * t141 * t143;
        let t147 = f64::powf(t146, 0.24974e-1);
        let t148 = t27 * t147;
        let t150 = 1.0 + 0.41666666666666666666e-9 * t112;
        let t151 = 1.0 / t150;
        let t152 = t148 * t151;
        let t155 = piecewise3(t96, 0.0, -3.0 / 8.0 * t105 * t152);
        let tzk0 = t95 + t155;
        zk[ip] += tzk0;
    }
}
