//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 730/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk730(t2206: f64, t3867: f64, t2289: f64, t3827: f64, t3857: f64, t3802: f64, t4394: f64, t3824: f64, t6: f64) -> (f64, f64, f64, f64, f64) {
    let t11493 = t2206 * t3867;
    let t11495 = t2289 * t3827;
    let t11497 = t2289 * t3857;
    let t11499 = t3802 * t4394;
    let t11514 = t6 * t3824;
    (t11493, t11495, t11497, t11499, t11514)
}
