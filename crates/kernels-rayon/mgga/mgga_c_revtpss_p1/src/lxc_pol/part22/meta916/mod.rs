//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta916 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta916(t3057: f64, t4930: f64, t15886: f64, t378: f64, t3046: f64, t1072: f64, t1647: f64, t3259: f64, t1071: f64, t15669: f64, t15654: f64, t12050: f64, t15907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55413, t55416, t55421, t55458, t55461, t55464, t55475, t55499) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3125(t3057, t4930, t15886, t378, t3046, t1072, t1647, t3259, t1071, t15669, t15654, t12050, t15907);
    (t55413, t55416, t55421, t55458, t55461, t55464, t55475, t55499)
}
