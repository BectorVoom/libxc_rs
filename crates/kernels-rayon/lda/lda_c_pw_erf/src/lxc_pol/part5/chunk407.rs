//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 407/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk407(t169: f64, t301: f64, t717: f64, t865: f64, t1045: f64, t1066: f64, t1069: f64, t1767: f64, t1774: f64, t1776: f64, t1778: f64, t1780: f64, t1800: f64, t1801: f64, t910: f64, t916: f64, t938: f64) -> (f64, f64) {
    let t1885 = t169 * t717 * t865 * t301;
    let t1887 = t910 - t916 + t938 - t1767 - t1774 + t1776 + t1778 - t1780 + t1800 + t1066 - t1069 - t1801 - t1045;
    (t1885, t1887)
}
