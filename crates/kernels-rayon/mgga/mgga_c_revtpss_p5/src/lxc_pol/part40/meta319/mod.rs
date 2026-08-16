//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1095;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta319(t11200: f64, t225: f64, t127: f64, t3218: f64, t371: f64, t1025: f64, t1058: f64, t3191: f64, t1021: f64, t3201: f64, t3231: f64, t1054: f64, t2434: f64, t373: f64, t367: f64, t3123: f64, t3168: f64, t3124: f64, t3173: f64, t1065: f64, t675: f64, t247: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11940, t11952, t11954, t11956, t11965, t11967) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1095(t11200, t225, t127, t3218, t371, t1025, t1058, t3191, t1021, t3201, t3231, t1054);
        let (t11972, t11977, t11980, t11986, t11988) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1096(t2434, t371, t373, t367, t3123, t3168, t3124, t3173, t1065, t675, t247, t906);
    (t11940, t11952, t11954, t11956, t11965, t11967, t11972, t11977, t11980, t11986, t11988)
}
