//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 935/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk935(t16973: f64, t17444: f64, t5400: f64, t639: f64, t1652: f64, t5406: f64, t1898: f64, t17420: f64, t17425: f64, t17430: f64, t17432: f64, t17434: f64, t17436: f64, t17439: f64, t17443: f64) -> (f64, f64, f64, f64) {
    let t17448 = 128.0_f64 / 27.0_f64 * t639 * t5400 * t17444 * t16973;
    let t17449 = t5406 * t1652;
    let t17450 = 32.0_f64 / 45.0_f64 * t17449;
    let t17452 = 16.0_f64 / 15.0_f64 * t5406 * t1898;
    let t17453 = -t17420 - t17425 - t17430 + t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452;
    (t17448, t17450, t17452, t17453)
}
