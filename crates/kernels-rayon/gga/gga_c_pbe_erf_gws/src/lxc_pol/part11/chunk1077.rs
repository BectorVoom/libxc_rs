//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1077/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1077(t12723: f64, t30876: f64, t1820: f64, t1821: f64, t41787: f64, t950: f64, t10442: f64, t3342: f64, t587: f64, t12464: f64, t2559: f64, t995: f64) -> (f64, f64, f64, f64, f64) {
    let t47343 = 32.0_f64 / 15.0_f64 * t30876 * t12723;
    let t47347 = 32.0_f64 / 45.0_f64 * t1820 * t1821 * t41787 * t950;
    let t47348 = t10442 * t3342;
    let t47351 = 16.0_f64 / 5.0_f64 * t587 * t1821 * t47348;
    let t47355 = 64.0_f64 / 9.0_f64 * t1820 * t2559 * t12464 * t995;
    (t47343, t47347, t47348, t47351, t47355)
}
