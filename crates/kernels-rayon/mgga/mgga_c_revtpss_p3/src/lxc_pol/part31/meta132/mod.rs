//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk721;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk722;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk723;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk724;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk725;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta132(t1086: f64, t378: f64, t994: f64, t1071: f64, t359: f64, t3140: f64, t3143: f64, t342: f64, t335: f64, t368: f64, t3153: f64, t3154: f64, t1035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3286, t3287, t3291, t3298) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk721(t1086, t378, t994, t1071, t359, t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk722(t3298, t342);
        let t3302 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk723(t335, t368);
        let t3303 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk724(t3153, t3302);
        let (t3304, t3316) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk725(t3154, t3303, t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk726(t3316, t342);
    (t3286, t3287, t3291, t3298, t3299, t3302, t3303, t3304, t3316, t3317)
}
