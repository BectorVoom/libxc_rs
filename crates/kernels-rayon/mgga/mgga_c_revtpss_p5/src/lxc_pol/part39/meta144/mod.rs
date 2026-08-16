//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk676;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta144(t1026: f64, t127: f64, t371: f64, t1025: f64, t3075: f64, t373: f64, t372: f64, t225: f64, t3046: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t3215, t3216, t3218, t3220, t3223) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk676(t1026, t127, t371, t1025, t3075, t373, t372, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk677(t3223, t366);
    (t3215, t3216, t3218, t3220, t3223, t3224)
}
