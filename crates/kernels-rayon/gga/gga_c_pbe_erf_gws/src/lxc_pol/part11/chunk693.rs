//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 693/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk693(t352: f64, t6126: f64, t6365: f64, t904: f64, t1112: f64, t2079: f64, t367: f64, t6553: f64, t899: f64, t4394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9296 = t352 * t6126;
    let t9343 = t6365 * t904;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9425 = t899 * t6553 * t367;
    let t9441 = t1112 * t4394;
    (t9296, t9343, t9385, t9386, t9425, t9441)
}
