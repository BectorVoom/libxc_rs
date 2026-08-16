//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1757;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta315(t10542: f64, t2801: f64, t231: f64, t2645: f64, t268: f64, t675: f64, t2798: f64, t760: f64, t9323: f64, t9318: f64, t2251: f64, t750: f64, t2611: f64, t2398: f64, t2615: f64, t2609: f64, t717: f64, t162: f64, t9544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10543, t10547, t10548, t10552, t10554, t10555) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1757(t10542, t2801, t231, t2645, t268, t675, t2798, t760, t9323, t9318, t2251, t750);
        let (t10556, t10561, t10563, t10565) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1758(t10555, t2611, t2398, t2615, t2609, t717, t162, t9544);
    (t10543, t10547, t10548, t10552, t10554, t10555, t10556, t10561, t10563, t10565)
}
