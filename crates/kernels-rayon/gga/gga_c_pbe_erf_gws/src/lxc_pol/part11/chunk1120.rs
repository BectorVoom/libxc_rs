//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1120(t1010: f64, t40493: f64, t12440: f64, t30630: f64, t10848: f64, t3527: f64, t1006: f64, t12703: f64, t12576: f64, t2612: f64, t12560: f64, t7130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47862 = 16.0_f64 / 45.0_f64 * t40493 * t1010;
    let t47864 = 16.0_f64 / 5.0_f64 * t30630 * t12440;
    let t47866 = 8.0_f64 / 15.0_f64 * t10848 * t3527;
    let t47868 = 8.0_f64 / 15.0_f64 * t1006 * t12703;
    let t47870 = 32.0_f64 / 15.0_f64 * t2612 * t12576;
    let t47872 = 32.0_f64 / 5.0_f64 * t7130 * t12560;
    (t47862, t47864, t47866, t47868, t47870, t47872)
}
