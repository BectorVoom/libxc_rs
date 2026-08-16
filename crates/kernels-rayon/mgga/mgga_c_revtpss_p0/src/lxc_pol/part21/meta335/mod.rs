//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta335(t11151: f64, t2908: f64, t141: f64, t11160: f64, t930: f64, t11132: f64, t240: f64, t624: f64, t281: f64, t283: f64, t2909: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11328, t11329, t11331, t11332, t11334, t11335, t11337, t11338, t11339) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1647(t11151, t2908, t141, t11160, t930, t11132, t240, t624, t281, t283, t2909, t698);
    (t11328, t11329, t11331, t11332, t11334, t11335, t11337, t11338, t11339)
}
