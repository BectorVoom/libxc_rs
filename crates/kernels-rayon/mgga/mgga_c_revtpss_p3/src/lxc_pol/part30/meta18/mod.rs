//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta18 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk129;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk130;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk131;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk132;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk133;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta18(t273: f64, t335: f64, t136: f64, t44: f64, t271: f64, t221: f64, t65: f64, t225: f64, t336: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t338, t340, t341) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk129(t273);
        let t342 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk130(t338, t341);
        let (t343, t344) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk131(t335, t136);
        let (t345, t346) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk132(t344, t44, t271);
        let (t348, t351) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk133(t221, t346, t65, t225, t342);
        let (t354, t355) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk134(t336, t73);
    (t338, t340, t341, t342, t343, t344, t345, t346, t348, t351, t354, t355)
}
