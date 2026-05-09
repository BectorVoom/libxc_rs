//! LDA_C_1D_CSC exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 48 shared lines across all orders.
//! Delta: 48 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_C_1D_CSC exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_1d_csc_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_ferro_0: f64,
    param_ferro_1: f64,
    param_ferro_2: f64,
    param_ferro_3: f64,
    param_ferro_4: f64,
    param_ferro_5: f64,
    param_ferro_6: f64,
    param_ferro_7: f64,
    param_ferro_8: f64,
    param_ferro_9: f64,
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (48 lines) ---
        let t1 = rho0 + rho1;
        let t2 = 1.0 / t1;
        let t3 = t2 / 2.0;
        let t4 = param_para_4;
        let t5 = t1 * t1;
        let t6 = 1.0 / t5;
        let t9 = t3 + t4 * t6 / 4.0;
        let t10 = param_para_7;
        let t14 = param_para_9;
        let t15 = f64::powf(t3, t14);
        let t16 = param_para_8 * t15;
        let t17 = 1.0 + t10 * t2 / 2.0 + t16;
        let t18 = f64::ln(t17);
        let t19 = t9 * t18;
        let t22 = param_para_1;
        let t25 = param_para_5;
        let t26 = f64::powf(t3, t25);
        let t27 = param_para_2 * t26;
        let t30 = param_para_6;
        let t31 = f64::powf(t3, t30);
        let t32 = param_para_3 * t31;
        let t34 = t22 * t2 + 2.0 * t27 + 2.0 * t32 + 2.0 * param_para_0;
        let t35 = 1.0 / t34;
        let t36 = t19 * t35;
        let t37 = param_ferro_4;
        let t40 = t3 + t37 * t6 / 4.0;
        let t41 = param_ferro_7;
        let t45 = param_ferro_9;
        let t46 = f64::powf(t3, t45);
        let t47 = param_ferro_8 * t46;
        let t48 = 1.0 + t41 * t2 / 2.0 + t47;
        let t49 = f64::ln(t48);
        let t50 = t40 * t49;
        let t53 = param_ferro_1;
        let t56 = param_ferro_5;
        let t57 = f64::powf(t3, t56);
        let t58 = param_ferro_2 * t57;
        let t61 = param_ferro_6;
        let t62 = f64::powf(t3, t61);
        let t63 = param_ferro_3 * t62;
        let t65 = t53 * t2 + 2.0 * t58 + 2.0 * t63 + 2.0 * param_ferro_0;
        let t66 = 1.0 / t65;
        let t68 = -t50 * t66 + t36;
        let t69 = rho0 - rho1;
        let t70 = t69 * t69;
        let t71 = t68 * t70;
        let t72 = t71 * t6;
        let tzk0 = -t36 + t72;
        zk[ip] += tzk0;
    }
}
