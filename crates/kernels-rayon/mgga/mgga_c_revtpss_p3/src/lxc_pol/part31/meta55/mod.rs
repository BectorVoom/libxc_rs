//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta55 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk354;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk355;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk356;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk357;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk358;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk359;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk360;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta55(t1044: f64, t1045: f64, t1042: f64, t362: f64, t39: f64, t40: f64, t361: f64, t351: f64, t127: f64, t371: f64, t373: f64, t367: f64, t365: f64, t369: f64, t270: f64, t283: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1046, t1047) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk354(t1044, t1045, t1042);
        let t1052 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk355(t362, t39, t40);
        let t1053 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk356(t1052, t361);
        let (t1054, t1058) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk357(t1053, t351, t127, t371, t373);
        let (t1060, t1061, t1062) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk358(t1058, t367, t365, t369, t361);
        let t1063 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk359(t1062, t351);
        let t1065 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk360(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk361(t1065, t66);
    (t1046, t1047, t1052, t1053, t1054, t1058, t1060, t1061, t1062, t1063, t1065, t1066)
}
