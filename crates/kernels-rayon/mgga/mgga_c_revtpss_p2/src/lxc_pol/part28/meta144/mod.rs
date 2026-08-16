//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk785;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta144(t3153: f64, t3154: f64, t3152: f64, t1042: f64, t1036: f64, t3148: f64, t3141: f64, t357: f64, t1038: f64, t1052: f64, t1033: f64, t127: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3167, t3168, t3169) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk785(t3153, t3154, t3152, t1042, t1036, t3148, t3141, t357, t1038, t1052, t1033);
        let t3172 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk786(t127, t246);
    (t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3167, t3168, t3169, t3172)
}
