//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta14 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk106;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk107;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk108;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk109;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk110;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk111;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta14(t239: f64, t240: f64, t206: f64, t137: f64, t72: f64, t125: f64, t217: f64, t222: f64, t237: f64, t225: f64, t234: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t241 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk106(t239, t240);
        let (t242, t243) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk107(t206);
        let (t244, t245) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk108(t241, t243, t137);
        let (t246, t247) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk109(t245, t72, t125);
        let (t248, t251) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk110(t244, t247, t217, t222, t237);
        let t252 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk111(t225, t251);
        let (t253, t256, t257) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk112(t234, t251, t213);
    (t241, t242, t243, t245, t246, t247, t248, t251, t252, t253, t256, t257)
}
