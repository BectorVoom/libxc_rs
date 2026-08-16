//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2227/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2227(t16750: f64, t482: f64, t371: f64, t372: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64) -> (f64, f64, f64, f64) {
    let t17278 = t482 * t16750;
    let t17280 = t371 * t372 * t17278;
    let t17283 = t3666 * t1803;
    let t17288 = t5215 * t1208;
    (t17278, t17280, t17283, t17288)
}
