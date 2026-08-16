//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk761;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk762;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk763;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk764;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta112(t239: f64, t2719: f64, t820: f64, t836: f64, t231: f64, t827: f64, t828: f64, t159: f64, t243: f64, t216: f64, t124: f64, t2394: f64, t800: f64, t2712: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2721, t2722) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk761(t239, t2719, t820, t836);
        let t2723 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk762(t231);
        let t2724 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk763(t2722, t2723);
        let (t2726, t2729, t2730) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk764(t2724, t827, t828, t159, t243, t216);
        let (t2732, t2735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk765(t124, t2394, t800, t2712, t785);
    (t2721, t2722, t2723, t2724, t2726, t2729, t2730, t2732, t2735)
}
