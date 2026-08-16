//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1002/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1002(t4561: f64, t822: f64, t2438: f64, t925: f64, t2434: f64, t325: f64, t6561: f64, t6504: f64, t4606: f64, t6507: f64, t6532: f64, t348: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15764 = t822 * t4561;
    let t15777 = t925 * t2438;
    let t15779 = t925 * t2434;
    let t15788 = t325 * t6561;
    let t15798 = t325 * t6504;
    let t15800 = t4606 * t6507;
    let t15820 = t325 * t6532;
    let t15824 = t739 * t348;
    (t15764, t15777, t15779, t15788, t15798, t15800, t15820, t15824)
}
