//! MGGA_X_MBRXH_BG vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 50 shared lines across all orders.
//! Delta: 72 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mbrxh_bg_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (50 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = tau[ip] * t23;
        let t25 = t15 * t15;
        let t27 = 1.0 / t25 / rho[ip];
        let t30 = M_CBRT6;
        let t31 = t30 * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t37 = sigma[ip] * t23;
        let t38 = rho[ip] * rho[ip];
        let t40 = 1.0 / t25 / t38;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t22;
        let t45 = t38 * t38;
        let t46 = t45 * rho[ip];
        let t48 = 1.0 / t15 / t46;
        let t51 = 0.46864e0 * t24 * t27 - 3.0 / 10.0 * t31 * t34 + 0.89e-1 * t37 * t40 + 0.106e-1 * t44 * t48;
        let t52 = f64::abs(t51);
        let t53 = t52 < 0.5e-12;
        let t54 = 0.0 < t51;
        let t55 = piecewise3(t54, 0.5e-12, -0.5e-12);
        let t56 = piecewise3(t53, t55, t51);
        let t57 = xc_mgga_x_br89_get_x(t56);
        let t59 = f64::exp(t57 / 3.0);
        let t60 = t21 * t59;
        let t61 = f64::exp(-t57);
        let t63 = 1.0 + t57 / 2.0;
        let t64 = t61 * t63;
        let t65 = 1.0 - t64;
        let t66 = 1.0 / t57;
        let t67 = t65 * t66;
        let t68 = t60 * t67;
        let t71 = piecewise3(t3, 0.0, -t20 * t68 / 4.0);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (72 lines) ---
        let t74 = t14 / t25 * t19;
        let t77 = M_CBRTPI;
        let t78 = t77 * t77;
        let t79 = t21 * t78;
        let t80 = piecewise3(t54, 0.0, 0.0);
        let t83 = t38 * rho[ip];
        let t85 = 1.0 / t25 / t83;
        let t88 = t45 * t38;
        let t90 = 1.0 / t15 / t88;
        let t94 = piecewise3(t53, t80, -0.78106666666666666667e0 * t24 * t40 - 0.23733333333333333333e0 * t37 * t85 - 0.56533333333333333333e-1 * t44 * t90);
        let t95 = t79 * t94;
        let t96 = t20 * t95;
        let t97 = t56 * t56;
        let t98 = 1.0 / t97;
        let t100 = f64::exp(-2.0 / 3.0 * t57);
        let t101 = 1.0 / t100;
        let t102 = t98 * t101;
        let t103 = t57 * t57;
        let t105 = t103 - 2.0 * t57 + 3.0;
        let t106 = 1.0 / t105;
        let t107 = t102 * t106;
        let t108 = t57 - 2.0;
        let t109 = t108 * t108;
        let t110 = t109 * t59;
        let t111 = t110 * t67;
        let t112 = t107 * t111;
        let t115 = t78 * t94;
        let t116 = t115 * t102;
        let t117 = t106 * t109;
        let t118 = t117 * t64;
        let t120 = t115 * t98;
        let t121 = t101 * t106;
        let t122 = t109 * t61;
        let t123 = t121 * t122;
        let t126 = t116 * t118 - t120 * t123 / 2.0;
        let t127 = t126 * t66;
        let t128 = t60 * t127;
        let t131 = t60 * t65;
        let t132 = t20 * t131;
        let t133 = 1.0 / t103;
        let t134 = t133 * t78;
        let t136 = t102 * t117;
        let t137 = t134 * t94 * t136;
        let t141 = piecewise3(t3, 0.0, -t74 * t68 / 12.0 - t96 * t112 / 12.0 - t20 * t128 / 4.0 + t132 * t137 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t141 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t144 = t23 * t40;
        let t146 = sigma[ip] * t22;
        let t150 = piecewise3(t53, t80, 0.89e-1 * t144 + 0.212e-1 * t146 * t48);
        let t151 = t79 * t150;
        let t152 = t20 * t151;
        let t155 = t78 * t150;
        let t156 = t155 * t102;
        let t158 = t155 * t98;
        let t161 = t156 * t118 - t158 * t123 / 2.0;
        let t162 = t161 * t66;
        let t163 = t60 * t162;
        let t167 = t134 * t150 * t136;
        let t171 = piecewise3(t3, 0.0, -t152 * t112 / 12.0 - t20 * t163 / 4.0 + t132 * t167 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t171;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t175 = piecewise3(t53, t80, 0.46864e0 * t23 * t27);
        let t176 = t79 * t175;
        let t177 = t20 * t176;
        let t180 = t78 * t175;
        let t181 = t180 * t102;
        let t183 = t180 * t98;
        let t186 = t181 * t118 - t183 * t123 / 2.0;
        let t187 = t186 * t66;
        let t188 = t60 * t187;
        let t192 = t134 * t175 * t136;
        let t196 = piecewise3(t3, 0.0, -t177 * t112 / 12.0 - t20 * t188 / 4.0 + t132 * t192 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t196;
        vtau[ip] += tvtau0;
    }
}
