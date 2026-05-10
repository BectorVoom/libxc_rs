//! GGA_C_OP_B88 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 63 shared lines across all orders.
//! Delta: 49 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_op_b88_vxc_unpol(
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
        // --- shared preamble (63 lines) ---
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
        let t20 = t16 / t18;
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
        let t32 = t23 * t31;
        let t33 = t23 * t23;
        let t34 = sigma[ip] * t33;
        let t35 = rho[ip] * rho[ip];
        let t36 = pow_1_3(rho[ip]);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = f64::sqrt(sigma[ip]);
        let t41 = t40 * t23;
        let t43 = 1.0 / t36 / rho[ip];
        let t45 = f64::ln(t41 * t43 + f64::sqrt(pow_2(t41 * t43) + 1.0));
        let t46 = t43 * t45;
        let t49 = 1.0 + 0.252e-1 * t41 * t46;
        let t50 = 1.0 / t49;
        let t55 = 1.0 + 0.93333333333333333332e-3 * t22 * t34 * t39 * t50;
        let t56 = 1.0 / t55;
        let t60 = piecewise3(t14, 0.0, t22 * t32 * t56 / 9.0);
        let t64 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t65 = piecewise5(t26, t5, t24, t6, -t7);
        let t66 = 1.0 + t65;
        let t67 = t66 * rho[ip];
        let t68 = pow_1_3(t67);
        let t69 = 1.0 / t68;
        let t70 = t23 * t69;
        let t74 = piecewise3(t64, 0.0, t22 * t70 * t56 / 9.0);
        let t75 = t60 + t74;
        let t76 = t75 == 0.0;
        let t77 = piecewise3(t76, f64::EPSILON, t75);
        let t80 = 0.36011538e1 / t77 + 0.5764e0;
        let t81 = t77 * t77;
        let t82 = t81 * t81;
        let t83 = 1.0 / t82;
        let t85 = t81 * t77;
        let t86 = 1.0 / t85;
        let t88 = 1.0 / t81;
        let t90 = 0.31390124030721e2 * t83 + 0.149643497914092e2 * t86 + 0.17833359087e1 * t88;
        let t91 = 1.0 / t90;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t80 * t91);
        zk[ip] += tzk0;
        // --- vxc delta (this level) (49 lines) ---
        let t95 = t9 * t80;
        let t99 = 1.0 / t30 / t29;
        let t105 = t55 * t55;
        let t106 = 1.0 / t105;
        let t107 = t35 * rho[ip];
        let t109 = 1.0 / t37 / t107;
        let t114 = t21 * sigma[ip];
        let t115 = t20 * t114;
        let t116 = t33 * t39;
        let t117 = t49 * t49;
        let t118 = 1.0 / t117;
        let t121 = 1.0 / t36 / t35 * t45;
        let t125 = t34 * t39 + 1.0;
        let t126 = f64::sqrt(t125);
        let t127 = 1.0 / t126;
        let t128 = t109 * t127;
        let t131 = -0.336e-1 * t41 * t121 - 0.336e-1 * t34 * t128;
        let t132 = t118 * t131;
        let t133 = t116 * t132;
        let t136 = -0.24888888888888888889e-2 * t22 * t34 * t109 * t50 - 0.93333333333333333332e-3 * t115 * t133;
        let t137 = t106 * t136;
        let t142 = piecewise3(t14, 0.0, -t22 * t23 * t99 * t56 * t28 / 27.0 - t22 * t32 * t137 / 9.0);
        let t144 = 1.0 / t68 / t67;
        let t154 = piecewise3(t64, 0.0, -t22 * t23 * t144 * t56 * t66 / 27.0 - t22 * t70 * t137 / 9.0);
        let t156 = piecewise3(t76, 0.0, t142 + t154);
        let t161 = t90 * t90;
        let t162 = 1.0 / t161;
        let t163 = t80 * t162;
        let t165 = 1.0 / t82 / t77;
        let t166 = t165 * t156;
        let t168 = t83 * t156;
        let t172 = -0.125560496122884e3 * t166 - 0.448930493742276e2 * t168 - 0.35666718174e1 * t86 * t156;
        let t177 = piecewise3(t4, 0.0, -0.25e0 * t95 * t91 + 0.90028845e0 * t10 * t88 * t156 * t91 + 0.25e0 * t10 * t163 * t172);
        let tvrho0 = rho[ip] * t177 + tzk0;
        vrho[ip] += tvrho0;
        let t183 = 1.0 / t40 * t23;
        let t188 = 0.126e-1 * t183 * t46 + 0.126e-1 * t116 * t127;
        let t189 = t118 * t188;
        let t190 = t116 * t189;
        let t193 = 0.93333333333333333332e-3 * t22 * t116 * t50 - 0.93333333333333333332e-3 * t115 * t190;
        let t194 = t106 * t193;
        let t198 = piecewise3(t14, 0.0, -t22 * t32 * t194 / 9.0);
        let t202 = piecewise3(t64, 0.0, -t22 * t70 * t194 / 9.0);
        let t204 = piecewise3(t76, 0.0, t198 + t202);
        let t209 = t165 * t204;
        let t211 = t83 * t204;
        let t213 = t86 * t204;
        let t215 = -0.125560496122884e3 * t209 - 0.448930493742276e2 * t211 - 0.35666718174e1 * t213;
        let t220 = piecewise3(t4, 0.0, 0.90028845e0 * t10 * t88 * t204 * t91 + 0.25e0 * t10 * t163 * t215);
        let tvsigma0 = rho[ip] * t220;
        vsigma[ip] += tvsigma0;
    }
}
