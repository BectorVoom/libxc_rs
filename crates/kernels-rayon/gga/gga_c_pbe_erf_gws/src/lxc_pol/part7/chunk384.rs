//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 384/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk384(t138: f64, t1570: f64, t1572: f64, t1577: f64, t1578: f64, t1590: f64, t514: f64, t520: f64, t101: f64) -> (f64, f64) {
    let t1592 = t138 * t1570 - 2.0_f64 * t1572 * t520 + 2.0_f64 * t1577 * t1578 - t1590 * t514;
    let t1593 = t101 * t1592;
    (t1592, t1593)
}
