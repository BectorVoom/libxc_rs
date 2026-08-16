//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1336;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta328(t11865: f64, t3090: f64, t3316: f64, t994: f64, t4891: f64, t1016: f64, t697: f64, t1011: f64, t11132: f64, t126: f64, t373: f64, t828: f64, t1086: f64, t3057: f64, t11671: f64, t3114: f64, t11200: f64, t225: f64, t1053: f64, t3204: f64, t1021: f64, t3201: f64, t1054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11866, t11875, t11881, t11890, t11921, t11922) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1336(t11865, t3090, t3316, t994, t4891, t1016, t697, t1011, t11132, t126, t373, t828);
        let (t11927, t11933, t11940, t11947, t11956, t11967) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1337(t1086, t3057, t3090, t11671, t3114, t11200, t225, t1053, t3204, t1021, t3201, t1054);
    (t11866, t11875, t11881, t11890, t11921, t11922, t11927, t11933, t11940, t11947, t11956, t11967)
}
