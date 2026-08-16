//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta661(t273: f64, t270: f64, t276: f64, t39484: f64, t9303: f64, t931: f64, t2922: f64, t275: f64, t2925: f64, t41306: f64, t11384: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2392(t273, t270, t276, t39484, t9303, t931, t2922, t275, t2925, t41306, t11384, t910);
    (t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41583)
}
