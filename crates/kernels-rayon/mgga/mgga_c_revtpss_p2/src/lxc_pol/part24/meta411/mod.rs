//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta411(t2925: f64, t41306: f64, t275: f64, t2872: f64, t2922: f64, t41245: f64, t315: f64, t41235: f64, t2941: f64, t2966: f64, t302: f64, t2969: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41502, t41520, t41549, t41588, t41592, t41610, t41658, t41667, t41672, t41690, t41740, t41741) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1353(t2925, t41306, t275, t2872, t2922, t41245, t315, t41235, t2941, t2966, t302, t2969);
    (t41502, t41520, t41549, t41588, t41592, t41610, t41658, t41667, t41672, t41690, t41740, t41741)
}
