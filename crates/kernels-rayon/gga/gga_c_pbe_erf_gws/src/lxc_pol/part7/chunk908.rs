//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 908/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk908(t5211: f64, t5523: f64, t617: f64, t7758: f64, t17090: f64, t17094: f64, t17098: f64, t17101: f64, t17103: f64, t17106: f64, t17110: f64, t17114: f64, t17117: f64, t17120: f64) -> (f64, f64) {
    let t17124 = 32.0_f64 / 9.0_f64 * t5211 * t7758 * t617 * t5523;
    let t17125 = t17090 - t17094 + t17098 + t17101 + t17103 + t17106 - t17110 - t17114 - t17117 - t17120 - t17124;
    (t17124, t17125)
}
