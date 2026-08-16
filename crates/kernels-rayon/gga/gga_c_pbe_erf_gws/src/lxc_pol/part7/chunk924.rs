//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 924/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk924(t17309: f64, t1648: f64, t5284: f64, t17285: f64, t17287: f64, t17291: f64, t17293: f64, t17297: f64, t17300: f64, t17302: f64, t17305: f64, t17308: f64) -> (f64, f64, f64) {
    let t17310 = 32.0_f64 / 45.0_f64 * t17309;
    let t17311 = t1648 * t5284;
    let t17312 = 32.0_f64 / 27.0_f64 * t17311;
    let t17313 = t17285 + t17287 + t17291 - t17293 - t17297 + t17300 + t17302 + t17305 + t17308 + t17310 + t17312;
    (t17310, t17312, t17313)
}
