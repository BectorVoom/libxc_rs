//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk712;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta129(t3153: f64, t3154: f64, t1036: f64, t3148: f64, t3141: f64, t357: f64, t1038: f64, t1052: f64, t1033: f64, t127: f64, t246: f64, t1046: f64, t1041: f64, t283: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3155, t3160, t3161, t3162, t3167, t3168, t3169, t3172) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk712(t3153, t3154, t1036, t3148, t3141, t357, t1038, t1052, t1033, t127, t246);
        let (t3173, t3174, t3181) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk713(t1046, t3172, t1041, t283, t905);
    (t3155, t3160, t3161, t3162, t3167, t3168, t3169, t3172, t3173, t3174, t3181)
}
