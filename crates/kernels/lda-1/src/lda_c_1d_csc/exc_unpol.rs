//! LDA_C_1D_CSC exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 23 shared lines across all orders.
//! Delta: 23 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_C_1D_CSC exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_1d_csc_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (23 lines) ---
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
    }
}
