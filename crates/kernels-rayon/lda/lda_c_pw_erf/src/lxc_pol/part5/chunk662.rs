//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 662/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk662(t5592: f64, t5594: f64, t156: f64, t1840: f64, t426: f64, t415: f64, t763: f64, t1859: f64, t443: f64, t1710: f64, t770: f64, t155: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5596 = 5.87616_f64 * t5592 * t5594;
    let t5598 = t426 * t156 * t1840;
    let t5607 = t415 * t763;
    let t5609 = 1.9486833333333333_f64 * t5607 * t5594;
    let t5618 = t1859 * t443;
    let t5621 = t770 * t1710;
    let t5639 = t155 * t436;
    (t5596, t5598, t5607, t5609, t5618, t5621, t5639)
}
