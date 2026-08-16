//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk571;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk572;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk573;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta110(t2710: f64, t2713: f64, t826: f64, t232: f64, t821: f64, t235: f64, t239: f64, t820: f64, t231: f64, t159: f64, t243: f64, t216: f64, t2712: f64, t785: f64, t225: f64, t849: f64, t823: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2716, t2718, t2719) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk571(t2710, t2713, t826, t232, t821, t235);
        let (t2721, t2723) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk572(t239, t2719, t820, t231);
        let (t2729, t2730, t2735) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk573(t159, t243, t216, t2712, t785);
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk574(t225, t2735, t826, t849, t820, t823, t843);
    (t2716, t2718, t2719, t2721, t2723, t2729, t2730, t2735, t2736, t2737, t2739, t2741)
}
