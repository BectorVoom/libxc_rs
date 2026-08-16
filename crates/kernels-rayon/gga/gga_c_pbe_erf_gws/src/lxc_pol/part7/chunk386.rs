//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 386/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk386(t1597: f64, t242: f64, t528: f64, t700: f64, t1354: f64, t41: f64) -> (f64, f64, f64) {
    let t1598 = t1597 * t242;
    let t1601 = 0.16752564107100880375e0_f64 * t528 * t700;
    let t1602 = t41 * t1354;
    (t1598, t1601, t1602)
}
