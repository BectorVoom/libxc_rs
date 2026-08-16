//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk779;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta147(t1020: f64, t1053: f64, t1021: f64, t1058: f64, t225: f64, t3043: f64, t366: f64, t371: f64, t373: f64, t676: f64, t367: f64, t3057: f64, t3059: f64, t372: f64, t1024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3191, t3194, t3196, t3197, t3201, t3203, t3204) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk779(t1020, t1053, t1021, t1058, t225, t3043, t366, t371, t373, t676, t367, t3057);
        let (t3205, t3206, t3208, t3211) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk780(t3204, t366, t3059, t373, t371, t372, t1024, t1053);
    (t3191, t3194, t3196, t3197, t3201, t3203, t3204, t3205, t3206, t3208, t3211)
}
