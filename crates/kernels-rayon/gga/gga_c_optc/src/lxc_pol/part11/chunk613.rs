//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 613/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk613(t277: f64, t364: f64, t4033: f64, t4783: f64, t4785: f64, t4817: f64, t4821: f64, t4851: f64, t4858: f64, t4900: f64, t4927: f64, t5053: f64, t5079: f64, t95: f64, t962: f64) -> f64 {
    let t5080 = t4783 + t4785 + t4817 + t4821 + t4033 / 3.0_f64 + t4851 * t364 / 2.0_f64 + t4858 + t4927 + 0.25844881434903430496e-2_f64 * t95 * t277 * t5053 * t962 - t4900 + t5079;
    t5080
}
