//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1042/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1042(t2880: f64, t6113: f64, t2884: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t916: f64, t2897: f64, t923: f64, t2908: f64, t6092: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6114 = t2880 * t6113;
    let t6120 = t2884 + 2.0_f64 / 9.0_f64 * t4571 - 2.0_f64 / 9.0_f64 * t6094 + 2.0_f64 / 3.0_f64 * t6098 - t6102 / 3.0_f64;
    let t6121 = t916 * t6120;
    let t6127 = t2897 * t6113;
    let t6129 = t923 * t6120;
    let t6132 = t2908 * t6092;
    (t6114, t6120, t6121, t6127, t6129, t6132)
}
