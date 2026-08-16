//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 931/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk931(t19803: f64, t825: f64, t2365: f64, t6158: f64, t2118: f64, t4422: f64, t328: f64, t6045: f64, t824: f64, t2306: f64, t4383: f64, t4395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19804 = t19803 * t825;
    let t19810 = t6158 * t2365;
    let t19817 = t2118 * t4422;
    let t19839 = t824 * t328 * t6045;
    let t19894 = t2306 * t4383;
    let t19898 = t4395 * t4383;
    (t19804, t19810, t19817, t19839, t19894, t19898)
}
