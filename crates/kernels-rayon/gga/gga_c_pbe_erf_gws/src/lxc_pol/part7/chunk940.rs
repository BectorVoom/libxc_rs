//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 940/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk940(t17500: f64, t4913: f64, t5456: f64, t1620: f64, t1621: f64, t1733: f64, t5454: f64, t155: f64, t213: f64, t1623: f64, t2591: f64, t644: f64) -> (f64, f64, f64, f64, f64) {
    let t17501 = 64.0_f64 / 405.0_f64 * t17500;
    let t17503 = 32.0_f64 / 5.0_f64 * t4913 * t5456;
    let t17507 = 16.0_f64 / 5.0_f64 * t1620 * t1621 * t5454 * t1733;
    let t17508 = t155 * t213;
    let t17510 = t1620 * t17508 * t1623;
    let t17511 = 32.0_f64 / 45.0_f64 * t17510;
    let t17512 = t2591 * t644;
    (t17501, t17503, t17507, t17511, t17512)
}
