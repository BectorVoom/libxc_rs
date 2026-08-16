//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta489(t72: f64, t8015: f64, t686: f64, t7058: f64, t7064: f64, t689: f64, t8011: f64, t25431: f64, t25411: f64, t786: f64, t7998: f64, t789: f64, t231: f64, t7997: f64, t836: f64, t7076: f64, t1558: f64, t7398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28359, t28360) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1742(t72, t8015, t686);
        let (t28361, t28366, t28368) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1743(t28360, t7058, t7064, t689, t8011);
        let (t28369, t28371, t28373, t28374, t28377, t28378, t28384) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744(t25431, t28368, t25411, t786, t7998, t789, t231, t7997, t836, t7076, t1558, t7398);
    (t28359, t28360, t28361, t28366, t28368, t28369, t28371, t28373, t28374, t28377, t28378, t28384)
}
