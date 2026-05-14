//! LDA_C_1D_CSC vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_C_1D_CSC vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_1d_csc_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_para_0: f64,
    param_para_1: f64,
    param_para_2: f64,
    param_para_3: f64,
    param_para_4: f64,
    param_para_5: f64,
    param_para_6: f64,
    param_para_7: f64,
    param_para_8: f64,
    param_para_9: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 / rho[ip];
        let t2 = t1 / 2.0;
        let t3 = param_para_4;
        let t4 = rho[ip] * rho[ip];
        let t5 = 1.0 / t4;
        let t8 = t2 + t3 * t5 / 4.0;
        let t9 = param_para_7;
        let t13 = param_para_9;
        let t14 = f64::powf(t2, t13);
        let t15 = param_para_8 * t14;
        let t16 = 1.0 + t9 * t1 / 2.0 + t15;
        let t17 = f64::ln(t16);
        let t18 = t8 * t17;
        let t21 = param_para_1;
        let t24 = param_para_5;
        let t25 = f64::powf(t2, t24);
        let t26 = param_para_2 * t25;
        let t29 = param_para_6;
        let t30 = f64::powf(t2, t29);
        let t31 = param_para_3 * t30;
        let t33 = t21 * t1 + 2.0 * t26 + 2.0 * t31 + 2.0 * param_para_0;
        let t34 = 1.0 / t33;
        let tzk0 = -t18 * t34;
        zk[ip] += tzk0;
        let t37 = 1.0 / t4 / rho[ip];
        let t40 = -t3 * t37 / 2.0 - t5 / 2.0;
        let t41 = rho[ip] * t40;
        let t42 = t17 * t34;
        let t44 = rho[ip] * t8;
        let t49 = -t9 * t5 / 2.0 - t15 * t13 * t1;
        let t50 = 1.0 / t16;
        let t52 = t49 * t50 * t34;
        let t54 = t33 * t33;
        let t55 = 1.0 / t54;
        let t56 = t17 * t55;
        let t64 = -2.0 * t26 * t24 * t1 - 2.0 * t31 * t29 * t1 - t21 * t5;
        let t65 = t56 * t64;
        let tvrho0 = -t41 * t42 - t44 * t52 + t44 * t65 + tzk0;
        vrho[ip] += tvrho0;
    }
}
