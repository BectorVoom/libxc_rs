//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 506/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk506(t1815: f64, t3406: f64, t639: f64, t2579: f64, t950: f64, t1821: f64, t1820: f64, t1000: f64, t1017: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3407 = t1815 * t3406;
    let t3409 = 8.0_f64 / 45.0_f64 * t639 * t3407;
    let t3410 = t2579 * t950;
    let t3411 = t1821 * t3410;
    let t3413 = 16.0_f64 / 45.0_f64 * t1820 * t3411;
    let t3414 = t1000 * t1017;
    (t3407, t3409, t3410, t3411, t3413, t3414)
}
