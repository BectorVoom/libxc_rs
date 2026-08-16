//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1347/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1347(t1972: f64, t4605: f64, t5322: f64, t6268: f64, t11821: f64, t806: f64, t2007: f64, t5187: f64, t1886: f64, t1980: f64, t2012: f64, t13727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17693 = 2.0_f64 / 45.0_f64 * t1972 * t4605;
    let t17695 = 8.0_f64 / 45.0_f64 * t6268 * t5322;
    let t17697 = 2.0_f64 / 45.0_f64 * t11821 * t806;
    let t17699 = 4.0_f64 / 45.0_f64 * t5187 * t2007;
    let t17702 = 8.0_f64 / 45.0_f64 * t1886 * t1980 * t2012;
    let t17703 = 8.0_f64 / 135.0_f64 * t13727;
    (t17693, t17695, t17697, t17699, t17702, t17703)
}
