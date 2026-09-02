//! LDA_C_1D_CSC exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_csc_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_para_4: f64,
    param_para_7: f64,
    param_para_9: f64,
    param_para_8: f64,
    param_para_1: f64,
    param_para_5: f64,
    param_para_2: f64,
    param_para_6: f64,
    param_para_3: f64,
    param_para_0: f64,
    param_ferro_4: f64,
    param_ferro_7: f64,
    param_ferro_9: f64,
    param_ferro_8: f64,
    param_ferro_1: f64,
    param_ferro_5: f64,
    param_ferro_2: f64,
    param_ferro_6: f64,
    param_ferro_3: f64,
    param_ferro_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 / rho[ip];
        let t2 = t1 / 2.0;
        let t3 = param_para_4;
        let t4 = rho[ip] * rho[ip];
        let t5 = 1.0 / t4;
        let t8 = t2 + t3 * t5 / 4.0;
        let t9 = param_para_7;
        let t13 = param_para_9;
        let t14 = rmath::pow(t2, t13);
        let t15 = param_para_8 * t14;
        let t16 = 1.0 + t9 * t1 / 2.0 + t15;
        let t17 = rmath::ln(t16);
        let t18 = t8 * t17;
        let t21 = param_para_1;
        let t24 = param_para_5;
        let t25 = rmath::pow(t2, t24);
        let t26 = param_para_2 * t25;
        let t29 = param_para_6;
        let t30 = rmath::pow(t2, t29);
        let t31 = param_para_3 * t30;
        let t33 = t21 * t1 + 2.0 * t26 + 2.0 * t31 + 2.0 * param_para_0;
        let t34 = 1.0 / t33;
        let tzk0 = -t18 * t34;
        zk[ip] += tzk0;
    }
}
