//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 834/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk834(t153: f64, t1962: f64, t4619: f64, t136: f64, t813: f64, t1601: f64, t497: f64, t1593: f64, t443: f64, t176: f64, t1988: f64, t4588: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6494 = t1962 * t153;
    let t6498 = t4619 * t153;
    let t6550 = t136 * t813;
    let t6559 = t1601 * t497;
    let t6636 = t1593 * t443;
    let t6747 = t1988 * t176;
    let t6751 = t4588 * t176;
    (t6494, t6498, t6550, t6559, t6636, t6747, t6751)
}
