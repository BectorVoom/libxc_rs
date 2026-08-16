//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2370;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta617(t10871: f64, t2645: f64, t234: f64, t39545: f64, t685: f64, t875: f64, t2760: f64, t2783: f64, t786: f64, t2801: f64, t10069: f64, t10920: f64, t231: f64, t2782: f64, t39709: f64, t10910: f64, t233: f64, t689: f64, t869: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40284, t40294, t40297, t40298, t40303) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2370(t10871, t2645, t234, t39545, t685, t875, t2760, t2783, t786, t2801, t10069, t10920);
        let (t40307, t40311, t40314, t40316) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371(t231, t2782, t2783, t39709, t10910, t233, t689, t869, t2778, t39515, t39501, t871);
    (t40284, t40294, t40297, t40298, t40303, t40307, t40311, t40314, t40316)
}
