//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta262 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1165;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1166;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1167;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1168;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1169;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1170;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1171;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta262(t351: f64, t7125: f64, t1058: f64, t1973: f64, t1061: f64, t1971: f64, t1017: f64, t1028: f64, t1047: f64, t1068: f64, t348: f64, t375: f64, t7106: f64, t7110: f64, t7111: f64, t7114: f64, t7117: f64, t7122: f64, t225: f64, t385: f64, t1976: f64, t342: f64, t1032: f64, t378: f64, t994: f64, t1078: f64, t359: f64, t988: f64, t1981: f64, t3056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7126, t7130, t7131) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1165(t351, t7125, t1058, t1973, t1061, t1971);
        let t7132 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1166(t351, t7131);
        let t7135 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1167(t1017, t1028, t1047, t1068, t348, t375, t7106, t7110, t7111, t7114, t7117, t7122, t7126, t7130, t7132);
        let (t7137, t7140) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1168(t225, t385, t7135, t1976, t342);
        let t7143 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1169(t1032, t378);
        let t7144 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1170(t7143, t994);
        let t7145 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1171(t1078, t359);
        let (t7147, t7150) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1172(t1976, t988, t7145, t1981, t3056);
    (t7126, t7130, t7131, t7132, t7135, t7137, t7140, t7143, t7144, t7145, t7147, t7150)
}
