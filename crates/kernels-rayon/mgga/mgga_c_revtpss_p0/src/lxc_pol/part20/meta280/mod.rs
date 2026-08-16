//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta280(t127: f64, t3206: f64, t371: f64, t3205: f64, t11200: f64, t225: f64, t366: f64, t11202: f64, t373: f64, t372: f64, t1053: f64, t3204: f64, t3218: f64, t1025: f64, t1058: f64, t3191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11937, t11938, t11940) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1139(t127, t3206, t371, t3205, t11200, t225);
        let (t11941, t11942, t11944, t11947, t11951, t11952, t11954) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1140(t11940, t366, t11202, t373, t371, t372, t1053, t3204, t127, t3218, t1025, t1058, t3191);
    (t11937, t11938, t11940, t11941, t11942, t11944, t11947, t11951, t11952, t11954)
}
