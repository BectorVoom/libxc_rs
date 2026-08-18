//! LDA_C_2D_PRM vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_prm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::powers::{pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_prm_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_N: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = f64::sqrt(rho[ip]);
        let t3 = f64::sqrt(M_PI);
        let t5 = 3.9274 * t1 + t3 / 2.0;
        let t6 = 1.0 / t5;
        let t7 = t1 * t6;
        let t9 = 3.9274 * t7 - 1.0;
        let t10 = t1 * t9;
        let t11 = 2.0 + param_c;
        let t12 = f64::sqrt(t11);
        let t13 = 1.0 / t12;
        let t15 = 0.3544538369424879 * t10 * t13;
        let t16 = 1.0 / t11;
        let t17 = t9 * t16;
        let t19 = 0.3999583253029731 * t7 * t17;
        let t20 = t5 * t5;
        let t21 = 1.0 / t20;
        let t23 = 1.0/pow_3_2(t11);
        let t25 = 0.17722691847124394 * t1 * t21 * t23;
        let t26 = 1.0 + param_c;
        let t27 = f64::sqrt(t26);
        let t28 = 1.0 / t27;
        let t30 = 0.7089076738849758 * t10 * t28;
        let t31 = 1.0 / t26;
        let t33 = 0.3999583253029731 * t7 * t31;
        let tzk0 = t15 + t19 + t25 + t30 + t33;
        zk[ip] += tzk0;
        let t34 = 1.0 / t1;
        let t35 = t34 * t9;
        let t36 = t35 * t13;
        let t38 = t34 * t6;
        let t41 = 1.9637 * t38 - 7.71223538 * t21;
        let t42 = t1 * t41;
        let t43 = t42 * t13;
        let t45 = t38 * t17;
        let t48 = t21 * t9 * t16;
        let t50 = t41 * t16;
        let t51 = t7 * t50;
        let t54 = t34 * t21 * t23;
        let t57 = 1.0 / t20 / t5;
        let t58 = t57 * t23;
        let t60 = t35 * t28;
        let t62 = t42 * t28;
        let t64 = t38 * t31;
        let t66 = t21 * t31;
        let t68 = 0.17722691847124394 * t36 + 0.3544538369424879 * t43 + 0.19997916265148655 * t45 - 0.7853981633974483 * t48 + 0.3999583253029731 * t51 + 0.08861345923562197 * t54 - 0.6960409996039635 * t58 + 0.3544538369424879 * t60 + 0.7089076738849758 * t62 + 0.19997916265148655 * t64 - 0.7853981633974483 * t66;
        let tvrho0 = rho[ip] * t68 + t15 + t19 + t25 + t30 + t33;
        vrho[ip] += tvrho0;
    }
}
