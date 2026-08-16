//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1196/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1196(t17579: f64, t17591: f64, t17594: f64, t2163: f64, t7007: f64, t15926: f64, t6958: f64, t518: f64, t7469: f64, t577: f64, t7465: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21694 = 16.0_f64 / 15.0_f64 * t17579;
    let t21695 = 16.0_f64 / 15.0_f64 * t17591;
    let t21696 = 8.0_f64 / 5.0_f64 * t17594;
    let t21698 = 8.0_f64 / 5.0_f64 * t7007 * t2163;
    let t21700 = 8.0_f64 / 5.0_f64 * t15926 * t6958;
    let t21701 = t7469 * t518;
    let t21703 = 8.0_f64 / 15.0_f64 * t21701 * t577;
    let t21704 = t7465 * t518;
    let t21706 = 8.0_f64 / 15.0_f64 * t21704 * t525;
    (t21694, t21695, t21696, t21698, t21700, t21703, t21706)
}
