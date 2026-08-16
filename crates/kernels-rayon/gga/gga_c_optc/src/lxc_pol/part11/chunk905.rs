//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 905/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk905(t3793: f64, t4920: f64, t845: f64, t16654: f64, t16657: f64, t16676: f64, t16828: f64, t16860: f64, t16864: f64, t16866: f64, t16869: f64, t16877: f64, t16949: f64, t16953: f64) -> (f64, f64, f64) {
    let t17041 = t3793 * t4920;
    let t17043 = 0.35089340384731224426e1_f64 * t845 * t17041;
    let t17044 = t16828 + t16860 + t16864 + t17043 - t16949 - t16953 - t16866 - t16657 + t16676 - t16869 - t16654 + t16877;
    (t17041, t17043, t17044)
}
