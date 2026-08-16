//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 533/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk533(t3096: f64, t619: f64, t3094: f64, t1026: f64, t628: f64, t205: f64, t126: f64, t95: f64) -> (f64, f64, f64, f64, f64) {
    let t3097 = t3096 * t619;
    let t3098 = t3094 * t3097;
    let t3100 = t628 * t1026;
    let t3101 = t3100 * t205;
    let t3103 = t126 * t95;
    (t3097, t3098, t3100, t3101, t3103)
}
