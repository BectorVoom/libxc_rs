//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta748(t11384: f64, t910: f64, t275: f64, t2872: f64, t2922: f64, t41245: f64, t41306: f64, t315: f64, t41235: f64, t11449: f64, t941: f64, t2941: f64, t2966: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t41583, t41588, t41592, t41610, t41658, t41662, t41667) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2821(t11384, t910, t275, t2872, t2922, t41245, t41306, t315, t41235, t11449, t941, t2941, t2966, t302);
    (t41583, t41588, t41592, t41610, t41658, t41662, t41667)
}
