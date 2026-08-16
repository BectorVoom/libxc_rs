//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta138 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk755;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk756;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk757;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk758;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk759;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk760;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk761;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk762;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta138(t378: f64, t989: f64, t340: f64, t992: f64, t338: f64, t999: f64, t996: f64, t1071: f64, t994: f64, t1096: f64, t1079: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3052, t3056) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk755(t378, t989, t340, t992);
        let t3057 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk756(t3056, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk757(t3057, t378);
        let t3059 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk758(t999);
        let t3060 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk759(t3059, t996);
        let t3063 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk760(t1071, t994);
        let (t3066, t3067) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk761(t1096, t999, t1079);
        let (t3070, t3075) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk762(t2846, t2848, t2855, t2860, t2864);
        let t3076 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk763(t3075, t996);
    (t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067, t3070, t3075, t3076)
}
