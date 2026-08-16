//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 379/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk379(t128: f64, t1533: f64, t10: f64, t125: f64, t390: f64, t1251: f64, t1: f64, t501: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1548 = t128 * t1533;
    let t1549 = t10 * t1548;
    let t1552 = t125 * t390;
    let t1553 = t1552 * t128;
    let t1555 = 0.16322666666666666667e0_f64 * t1553 * t1251;
    let t1557 = t501 * t506 * t1;
    (t1548, t1549, t1552, t1553, t1555, t1557)
}
