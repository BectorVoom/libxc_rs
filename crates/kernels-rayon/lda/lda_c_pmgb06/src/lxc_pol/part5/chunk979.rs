//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 979/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk979(t1447: f64, t6509: f64, t5499: f64, t6513: f64, t486: f64, t6610: f64, t5115: f64, t802: f64, t12981: f64, t6633: f64, t13007: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16522 = t1447 * t6509;
    let t16524 = t5499 * t6513;
    let t16535 = t486 * t6610;
    let t16537 = t802 * t5115;
    let t16542 = t12981 * t6633;
    let t16549 = t13007 * t6562;
    (t16522, t16524, t16535, t16537, t16542, t16549)
}
