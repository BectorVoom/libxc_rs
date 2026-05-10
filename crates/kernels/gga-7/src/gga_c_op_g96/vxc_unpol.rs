//! GGA_C_OP_G96 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 56 shared lines across all orders.
//! Delta: 42 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_op_g96_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (56 lines) ---
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t33 = f64::sqrt(sigma[ip]);
        let t34 = t33 * t23;
        let t35 = pow_1_3(rho[ip]);
        let t37 = 1.0 / t35 / rho[ip];
        let t38 = t34 * t37;
        let t39 = f64::sqrt(t38);
        let t40 = t39 * t38;
        let t44 = 1.0 + 2.0 / 1233.0 * t20 * t21 * t40;
        let t45 = 1.0 / t44;
        let t49 = piecewise3(t14, 0.0, t22 * t23 * t31 * t45 / 9.0);
        let t53 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t54 = piecewise5(t26, t5, t24, t6, -t7);
        let t55 = 1.0 + t54;
        let t56 = t55 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t63 = piecewise3(t53, 0.0, t22 * t23 * t58 * t45 / 9.0);
        let t64 = t49 + t63;
        let t65 = t64 == 0.0;
        let t66 = piecewise3(t65, f64::EPSILON, t64);
        let t69 = 0.359628532e1 / t66 + 0.5764e0;
        let t70 = t66 * t66;
        let t71 = t70 * t70;
        let t72 = 1.0 / t71;
        let t74 = t70 * t66;
        let t75 = 1.0 / t74;
        let t77 = 1.0 / t70;
        let t79 = 0.312207199195441936e2 * t72 + 0.149037398922132448e2 * t75 + 0.1778517305052e1 * t77;
        let t80 = 1.0 / t79;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t69 * t80);
        zk[ip] += tzk0;
        // --- vxc delta (this level) (42 lines) ---
        let t84 = t9 * t69;
        let t88 = 1.0 / t30 / t29;
        let t94 = t18 * t18;
        let t95 = 1.0 / t94;
        let t96 = t15 * t95;
        let t97 = t21 * t21;
        let t98 = t23 * t23;
        let t99 = t97 * t98;
        let t100 = t96 * t99;
        let t101 = t44 * t44;
        let t102 = 1.0 / t101;
        let t103 = t31 * t102;
        let t104 = t39 * t33;
        let t105 = rho[ip] * rho[ip];
        let t107 = 1.0 / t35 / t105;
        let t108 = t104 * t107;
        let t113 = piecewise3(t14, 0.0, -t22 * t23 * t88 * t45 * t28 / 27.0 + 4.0 / 3699.0 * t100 * t103 * t108);
        let t115 = 1.0 / t57 / t56;
        let t121 = t58 * t102;
        let t126 = piecewise3(t53, 0.0, -t22 * t23 * t115 * t45 * t55 / 27.0 + 4.0 / 3699.0 * t100 * t121 * t108);
        let t128 = piecewise3(t65, 0.0, t113 + t126);
        let t133 = t79 * t79;
        let t134 = 1.0 / t133;
        let t135 = t69 * t134;
        let t137 = 1.0 / t71 / t66;
        let t138 = t137 * t128;
        let t140 = t72 * t128;
        let t144 = -0.1248828796781767744e3 * t138 - 0.447112196766397344e2 * t140 - 0.3557034610104e1 * t75 * t128;
        let t149 = piecewise3(t4, 0.0, -0.25e0 * t84 * t80 + 0.89907133e0 * t10 * t77 * t128 * t80 + 0.25e0 * t10 * t135 * t144);
        let tvrho0 = rho[ip] * t149 + tzk0;
        vrho[ip] += tvrho0;
        let t151 = 1.0 / t33;
        let t152 = t39 * t151;
        let t153 = t152 * t37;
        let t157 = piecewise3(t14, 0.0, -t100 * t103 * t153 / 2466.0);
        let t161 = piecewise3(t53, 0.0, -t100 * t121 * t153 / 2466.0);
        let t163 = piecewise3(t65, 0.0, t157 + t161);
        let t168 = t137 * t163;
        let t170 = t72 * t163;
        let t172 = t75 * t163;
        let t174 = -0.1248828796781767744e3 * t168 - 0.447112196766397344e2 * t170 - 0.3557034610104e1 * t172;
        let t179 = piecewise3(t4, 0.0, 0.89907133e0 * t10 * t77 * t163 * t80 + 0.25e0 * t10 * t135 * t174);
        let tvsigma0 = rho[ip] * t179;
        vsigma[ip] += tvsigma0;
    }
}
