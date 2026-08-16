//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta643(t2925: f64, t41306: f64, t11545: f64, t914: f64, t2866: f64, t2923: f64, t11384: f64, t910: f64, t275: f64, t2872: f64, t2922: f64, t41245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41502, t41520, t41549, t41571, t41578, t41583, t41588, t41592) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2428(t2925, t41306, t11545, t914, t2866, t2923, t11384, t910, t275, t2872, t2922, t41245);
    (t41502, t41520, t41549, t41571, t41578, t41583, t41588, t41592)
}
