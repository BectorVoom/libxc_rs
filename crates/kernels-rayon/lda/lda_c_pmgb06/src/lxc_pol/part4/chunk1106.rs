//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1106/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1106(t2057: f64, t955: f64, t2054: f64, t405: f64, t5016: f64, t5019: f64, t4913: f64, t5022: f64, t5010: f64, t5013: f64, t12535: f64, t495: f64, t5065: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13619 = t955 * t2057;
    let t13621 = t955 * t2054;
    let t13633 = t405 * t5016;
    let t13635 = t405 * t5019;
    let t13637 = t4913 * t5022;
    let t13639 = t405 * t5010;
    let t13644 = t405 * t5013;
    let t13672 = t5065 * t12535 * t495;
    (t13619, t13621, t13633, t13635, t13637, t13639, t13644, t13672)
}
