//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta50 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk326;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk327;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk328;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk329;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk330;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta50(t953: f64, t954: f64, t902: f64, t908: f64, t324: f64, t320: f64, t315: f64, t928: f64, t919: f64, t924: f64, t932: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t955, t958, t960) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk326(t953, t954, t902, t908);
        let (t961, t963, t964) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk327(t324, t960, t320);
        let t965 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk328(t315, t964);
        let (t967, t970, t972) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk329(t902, t928, t908, t919, t924, t932);
        let t973 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk330(t323);
        let t974 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk331(t972, t973);
    (t955, t958, t960, t961, t963, t964, t965, t967, t970, t972, t973, t974)
}
