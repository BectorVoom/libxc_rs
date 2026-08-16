//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 421/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk421(t2048: f64, t88: f64, t1834: f64, t1838: f64, t1981: f64, t1985: f64, t1988: f64, t1992: f64, t1996: f64, t2000: f64, t2044: f64, t2047: f64) -> f64 {
    let t2050 = 32.0_f64 * t2048 * t88;
    let t2051 = -t1996 - t2000 - t1988 + t2044 + t2047 - t2050 - t1834 + t1992 - t1981 - t1838 + t1985;
    t2051
}
