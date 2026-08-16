//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta41 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk262;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk263;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk264;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk265;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk266;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk267;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta41(t787: f64, t789: f64, t579: f64, t65: f64, t64: f64, t159: f64, t222: f64, t228: f64, t216: f64, t136: f64, t220: f64, t124: f64, t775: f64, t212: f64, t27: f64, t235: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t791, t793) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk262(t787, t789, t579, t65);
        let t794 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk263(t64, t793);
        let (t795, t797, t798, t799) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk264(t159, t794, t222, t228, t216);
        let t800 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk265(t136, t220);
        let t802 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk266(t124, t775, t800);
        let t807 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk267(t124, t212, t27, t800);
        let t808 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk268(t235, t240);
    (t791, t793, t794, t795, t797, t798, t799, t800, t802, t807, t808)
}
