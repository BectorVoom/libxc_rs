//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1346;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1347;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1348;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1349;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta337(t11132: f64, t2435: f64, t907: f64, t2854: f64, t689: f64, t2859: f64, t2863: f64, t159: f64, t3181: f64, t2851: f64, t631: f64, t45: f64, t1071: f64, t3057: f64, t3259: f64, t994: f64, t342: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1346(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1347(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1348(t2859, t689);
        let t11140 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1349(t2863, t689);
        let (t11142, t11144, t11150, t11187, t11190, t11195, t11200) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1350(t159, t3181, t2851, t631, t45, t1071, t3057, t3259, t994, t342, t992, t338);
    (t11133, t11134, t11136, t11138, t11140, t11142, t11144, t11150, t11187, t11190, t11195, t11200)
}
