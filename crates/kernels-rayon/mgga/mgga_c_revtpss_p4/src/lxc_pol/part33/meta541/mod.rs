//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta541(t225: f64, t29109: f64, t494: f64, t1769: f64, t7627: f64, t7637: f64, t11239: f64, t1276: f64, t3596: f64, t2149: f64, t29157: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29183, t29186, t29187, t29192, t29193, t29194, t29195) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1912(t225, t29109, t494, t1769, t7627, t7637, t11239, t1276, t3596, t2149, t29157, t3153);
    (t29183, t29186, t29187, t29192, t29193, t29194, t29195)
}
