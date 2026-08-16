//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk338;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk339;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk340;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk341;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk342;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk343;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk344;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk345;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta52(t225: f64, t385: f64, t902: f64, t908: f64, t344: f64, t614: f64, t139: f64, t221: f64, t346: f64, t345: f64, t220: f64, t44: f64, t124: f64, t65: f64, t270: f64, t271: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t996 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk338(t225, t385);
        let (t997, t999) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk339(t902, t908);
        let t1000 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk340(t996, t999);
        let (t1003, t1007) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk341(t344, t614, t139, t221, t346);
        let (t1009, t1010) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk342(t1007, t345, t220, t344);
        let t1011 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk343(t1010, t44);
        let t1012 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk344(t124, t65);
        let t1014 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk345(t270, t271);
        let t1015 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk346(t1014, t905);
    (t996, t997, t999, t1000, t1003, t1007, t1009, t1010, t1011, t1012, t1014, t1015)
}
