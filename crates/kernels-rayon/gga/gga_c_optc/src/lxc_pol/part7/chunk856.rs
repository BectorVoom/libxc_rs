//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 856/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk856(t2797: f64, t2800: f64, t7274: f64, t866: f64, t930: f64, t288: f64, t875: f64, t2606: f64, t3813: f64, t2663: f64, t277: f64, t115: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8180 = t2797 * t2800;
    let t8182 = t7274 * t866;
    let t8183 = t930 * t8182;
    let t8185 = t288 * t875;
    let t8186 = t3813 * t2606;
    let t8187 = t8185 * t8186;
    let t8191 = 1.0_f64 / t2663 / t277;
    let t8192 = t8191 * t115;
    (t8180, t8182, t8183, t8185, t8187, t8192)
}
