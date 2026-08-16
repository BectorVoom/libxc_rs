//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk79;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk80;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk81;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk82;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk83;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk84;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk85;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta11(t158: f64, t190: f64, t157: f64, t162: f64, t187: f64, t73: f64, t152: f64, t45: f64, t57: f64, t78: f64, t81: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t191, t192, t194, t196) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk79(t158, t190, t157, t162, t187);
        let t197 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk80(t73);
        let t198 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk81(t196, t197);
        let t199 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk82(t152);
        let (t200, t202, t205) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk83(t45, t57, t78, t199, t81, zeta_threshold);
        let t206 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk84(t205);
        let t207 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk85(t205, t206);
    (t191, t192, t194, t196, t197, t198, t199, t200, t202, t205, t206, t207)
}
