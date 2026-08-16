//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk790;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk791;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk792;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta185(t225: f64, t4028: f64, t4043: f64, t1412: f64, t73: f64, t3829: f64, t1394: f64, t3889: f64, t1392: f64, t1395: f64, t539: f64, t541: f64, t543: f64, t1390: f64, t828: f64, t1389: f64, t1408: f64, t2736: f64, t1388: f64, t1410: f64, t3970: f64, t3976: f64, t3982: f64, t3987: f64, t3990: f64, t3996: f64, t4002: f64, t4006: f64, t4014: f64, t4022: f64, t1370: f64, t3926: f64, t3931: f64, t3934: f64, t3940: f64, t3944: f64, t3946: f64, t3950: f64, t3953: f64, t3956: f64, t3958: f64, t3961: f64, t3967: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4045, t4050, t4053, t4056) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk790(t225, t4028, t4043, t1412, t73, t3829, t1394, t3889, t1392, t1395, t539, t541);
        let t4057 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk791(t4056, t543);
        let (t4059, t4062, t4064, t4065) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk792(t1390, t4057, t828, t1389, t1408, t2736, t1388, t1410, t3970, t3976, t3982, t3987, t3990, t3996, t4002, t4006, t4014, t4022);
        let t4066 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk793(t1370, t1388, t3926, t3931, t3934, t3940, t3944, t3946, t3950, t3953, t3956, t3958, t3961, t3967, t4065);
    (t4045, t4050, t4053, t4056, t4057, t4059, t4062, t4064, t4066)
}
