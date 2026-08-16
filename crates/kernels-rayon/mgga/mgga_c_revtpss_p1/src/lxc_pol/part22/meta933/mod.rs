//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta933 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta933(t12916: f64, t17780: f64, t5331: f64, t1260: f64, t45385: f64, t12640: f64, t17728: f64, t489: f64, t12744: f64, t17350: f64, t3781: f64, t5219: f64, t5330: f64, t17743: f64, t3718: f64, t12881: f64, t5391: f64, t1222: f64, t16720: f64, t17471: f64, t17753: f64, t17755: f64, t12800: f64, t5378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57336, t57344, t57348, t57378, t57382) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163(t12916, t17780, t5331, t1260, t45385, t12640, t17728, t489, t12744, t17350, t3781, t5219, t5330);
        let (t57386, t57421, t57428, t57435, t57449) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164(t12916, t17743, t3718, t12881, t5391, t1222, t16720, t17471, t17753, t17755, t12800, t5378);
    (t57336, t57344, t57348, t57378, t57382, t57386, t57421, t57428, t57435, t57449)
}
