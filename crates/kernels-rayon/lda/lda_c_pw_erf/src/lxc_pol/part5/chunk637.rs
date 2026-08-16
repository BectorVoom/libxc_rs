//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 637/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk637(t219: f64, t4049: f64, t2010: f64, t3863: f64, t571: f64, t1949: f64, t3854: f64, t4062: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4776 = t4049 * t219;
    let t4788 = t3863 * t2010;
    let t4790 = 16.0_f64 / 135.0_f64 * t571 * t4788;
    let t4791 = t3854 * t1949;
    let t4793 = 32.0_f64 / 135.0_f64 * t571 * t4791;
    let t4794 = t4062 * t219;
    (t4776, t4788, t4790, t4791, t4793, t4794)
}
