//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 929/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk929(t17363: f64, t1620: f64, t1621: f64, t1724: f64, t5500: f64, t1640: f64, t1643: f64, t16986: f64, t639: f64, t1891: f64, t642: f64, t1648: f64, t5510: f64) -> (f64, f64, f64, f64, f64) {
    let t17364 = 32.0_f64 / 15.0_f64 * t17363;
    let t17368 = 8.0_f64 / 5.0_f64 * t1620 * t1621 * t5500 * t1724;
    let t17372 = 4.0_f64 / 9.0_f64 * t639 * t1640 * t1643 * t16986;
    let t17376 = 8.0_f64 / 15.0_f64 * t639 * t642 * t1891 * t16986;
    let t17378 = 32.0_f64 / 15.0_f64 * t1648 * t5510;
    (t17364, t17368, t17372, t17376, t17378)
}
