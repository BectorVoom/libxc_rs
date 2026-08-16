//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk878;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk879;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk880;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta184(t3088: f64, t4890: f64, t3299: f64, t1668: f64, t3153: f64, t1043: f64, t3154: f64, t3117: f64, t3317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t4891 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk878(t3088, t4890);
        let t4892 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk879(t3299, t4891);
        let t4893 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk880(t1668, t3153);
        let (t4894, t4895, t4896, t4899) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk881(t1043, t3154, t4893, t3117, t3317, t4891);
    (t4891, t4892, t4893, t4894, t4895, t4896, t4899)
}
