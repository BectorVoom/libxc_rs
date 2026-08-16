//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 705/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk705(t1122: f64, t4549: f64, t2148: f64, t980: f64, t968: f64, t2142: f64, t273: f64, t698: f64, t959: f64, t3768: f64, t3867: f64, t3871: f64, t3874: f64, t3892: f64, t3893: f64, t3895: f64, t3899: f64, t3904: f64, t3908: f64, t3911: f64) -> (f64, f64) {
    let t4550 = t4549 * t1122;
    let t4552 = t2148 * t980;
    let t4554 = t2148 * t968;
    let t4556 = t2142 * t273;
    let t4558 = 1.1696447245269292_f64 * t4556 * t698;
    let t4559 = t2148 * t959;
    let t4566 = 0.01084358130030174_f64 * t4550 + 1.1696447245269292_f64 * t4552 - 17.315859105681465_f64 * t4554 - t4558 - 0.5848223622634646_f64 * t4559 + 2.0_f64 * t3768 + t3892 - t3867 + t3871 - 16.0_f64 * t3893 - 4.0_f64 * t3895 - 4.0_f64 * t3899 + t3874 + t3904 + 40.0_f64 * t3908 + t3911;
    (t4556, t4566)
}
