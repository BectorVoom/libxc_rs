//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1851;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta353(t1053: f64, t3204: f64, t127: f64, t3218: f64, t371: f64, t1025: f64, t1058: f64, t3191: f64, t1021: f64, t3201: f64, t3231: f64, t1054: f64, t2434: f64, t373: f64, t367: f64, t3123: f64, t3168: f64, t3124: f64, t3173: f64, t1065: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11947, t11951, t11952, t11954, t11956, t11965, t11967) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1851(t1053, t3204, t127, t3218, t371, t1025, t1058, t3191, t1021, t3201, t3231, t1054);
        let (t11970, t11972, t11977, t11980, t11986) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1852(t2434, t371, t373, t367, t3123, t3168, t3124, t3173, t1065, t675);
    (t11947, t11951, t11952, t11954, t11956, t11965, t11967, t11970, t11972, t11977, t11980, t11986)
}
