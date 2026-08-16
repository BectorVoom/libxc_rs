//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk713;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta128(t3181: f64, t66: f64, t1020: f64, t1062: f64, t1021: f64, t1058: f64, t371: f64, t373: f64, t676: f64, t367: f64, t225: f64, t3057: f64, t366: f64, t1024: f64, t1053: f64, t1026: f64, t127: f64, t1025: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3182, t3188, t3194, t3201, t3203, t3204) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk713(t3181, t66, t1020, t1062, t1021, t1058, t371, t373, t676, t367, t225, t3057);
        let (t3205, t3211, t3215, t3216, t3223) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk714(t3204, t366, t1024, t1053, t1026, t127, t371, t1025, t225, t3046);
    (t3182, t3188, t3194, t3201, t3203, t3204, t3205, t3211, t3215, t3216, t3223)
}
