//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 769/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk769(t6120: f64, t916: f64, t2897: f64, t6113: f64, t923: f64, t2908: f64, t6092: f64, t141: f64, t6096: f64, t930: f64, t6100: f64, t2892: f64, t2905: f64, t4571: f64, t4620: f64, t6094: f64, t6098: f64, t6102: f64, t6114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6121 = t916 * t6120;
    let t6127 = t2897 * t6113;
    let t6129 = t923 * t6120;
    let t6132 = t2908 * t6092;
    let t6133 = t141 * t6132;
    let t6135 = t930 * t6096;
    let t6136 = t141 * t6135;
    let t6138 = t930 * t6100;
    let t6139 = t141 * t6138;
    let t6141 = -0.9494625e0_f64 * t6114 + 0.1898925e1_f64 * t6121 + t2892 + 0.19931111111111111111e0_f64 * t4571 - 0.19931111111111111111e0_f64 * t6094 + 0.59793333333333333334e0_f64 * t6098 - 0.29896666666666666667e0_f64 * t6102 + 0.15358125e0_f64 * t6127 + 0.3071625e0_f64 * t6129 + t2905 + 0.10954222222222222222e0_f64 * t4620 - 0.27385555555555555556e-1_f64 * t6133 + 0.16431333333333333333e0_f64 * t6136 - 0.82156666666666666667e-1_f64 * t6139;
    (t6121, t6127, t6129, t6132, t6133, t6135, t6136, t6138, t6139, t6141)
}
