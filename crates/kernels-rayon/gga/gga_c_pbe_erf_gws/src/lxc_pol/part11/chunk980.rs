//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 980/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk980(t1964: f64, t3380: f64, t11159: f64, t547: f64, t5621: f64, t985: f64, t10020: f64, t1396: f64, t10016: f64, t409: f64, t1444: f64, t9762: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33426 = t3380 * t1964;
    let t33431 = t11159 * t547;
    let t33446 = t5621 * t985;
    let t33523 = t10020 * t1396;
    let t33527 = t409 * t10016;
    let t33530 = t9762 * t1444;
    (t33426, t33431, t33446, t33523, t33527, t33530)
}
