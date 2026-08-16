//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta766(t12077: f64, t989: f64, t12153: f64, t3057: f64, t3043: f64, t3316: f64, t1071: f64, t11200: f64, t378: f64, t42358: f64, t11223: f64, t3376: f64, t3383: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43574, t43598, t43611, t43637, t43642, t43656, t43748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2848(t12077, t989, t12153, t3057, t3043, t3316, t1071, t11200, t378, t42358, t11223, t3376, t3383);
    (t43574, t43598, t43611, t43637, t43642, t43656, t43748)
}
