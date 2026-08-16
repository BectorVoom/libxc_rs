//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 671/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk671(t3768: f64, t3867: f64, t3871: f64, t3874: f64, t3877: f64, t3892: f64, t3893: f64, t3901: f64, t3906: f64, t3908: f64, t3911: f64, t4550: f64, t4552: f64, t4554: f64, t4558: f64, t4559: f64) -> f64 {
    let t6097 = 0.02168716260060348_f64 * t4550 + 2.3392894490538585_f64 * t4552 - 34.63171821136293_f64 * t4554 - t4558 - 1.1696447245269292_f64 * t4559 + t3768 + t3892 - t3867 + t3871 - 8.0_f64 * t3893 + t3874 - 8.0_f64 * t3901 + 32.0_f64 * t3906 + 20.0_f64 * t3908 + t3911 + t3877;
    t6097
}
