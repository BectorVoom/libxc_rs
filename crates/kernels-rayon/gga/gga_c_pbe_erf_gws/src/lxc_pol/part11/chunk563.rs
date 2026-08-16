//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 563/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk563(t2080: f64, t2085: f64, t3803: f64, t860: f64, t2079: f64, t3802: f64, t2306: f64, t905: f64, t3126: f64, t1105: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3805 = t2080 * t3803 * t2085;
    let t3807 = t3805 * t860 / 96.0_f64;
    let t3808 = t2079 * t3802;
    let t3809 = t3808 * t2306;
    let t3810 = t905 * t3809;
    let t3813 = 7.0_f64 / 144.0_f64 * t3126;
    let t3814 = t343 * t1105;
    (t3805, t3807, t3808, t3810, t3813, t3814)
}
