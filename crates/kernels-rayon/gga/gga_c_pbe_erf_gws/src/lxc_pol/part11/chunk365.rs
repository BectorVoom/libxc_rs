//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 365/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk365(t137: f64, t512: f64, t131: f64, t120: f64, t133: f64, t542: f64, t242: f64, t762: f64, t528: f64, t700: f64, t1383: f64, t148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1576 = 1.0_f64 / t512 / t137;
    let t1577 = t131 * t1576;
    let t1583 = 0.38316777777777777777e0_f64 * t133 * t542 * t120;
    let t1596 = 0.16752564107100880375e0_f64 * t762 * t242;
    let t1601 = 0.16752564107100880375e0_f64 * t528 * t700;
    let t1608 = 0.83762820535504401876e-1_f64 * t148 * t1383;
    (t1576, t1577, t1583, t1596, t1601, t1608)
}
