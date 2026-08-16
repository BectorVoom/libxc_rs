//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 937/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk937(t1648: f64, t5010: f64, t155: f64, t188: f64, t1820: f64, t1887: f64, t1620: f64, t5493: f64, t5501: f64, t1885: f64, t5175: f64, t5177: f64, t562: f64) -> (f64, f64, f64, f64) {
    let t17469 = 32.0_f64 / 9.0_f64 * t1648 * t5010;
    let t17470 = t155 * t188;
    let t17472 = t1820 * t17470 * t1887;
    let t17473 = 32.0_f64 / 45.0_f64 * t17472;
    let t17475 = t1620 * t5493 * t5501;
    let t17476 = 32.0_f64 / 15.0_f64 * t17475;
    let t17481 = 32.0_f64 / 5.0_f64 * t1820 * t1885 * t5175 * t5177 * t562;
    (t17469, t17473, t17476, t17481)
}
