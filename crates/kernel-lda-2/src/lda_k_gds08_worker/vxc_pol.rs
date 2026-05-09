//! LDA_K_GDS08_WORKER vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 30 shared lines across all orders.
//! Delta: 42 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_K_GDS08_WORKER vxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (30 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (42 lines) ---
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
