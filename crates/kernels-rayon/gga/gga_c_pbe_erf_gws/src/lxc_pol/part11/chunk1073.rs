//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1073/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1073(t1820: f64, t2559: f64, t30455: f64, t3342: f64, t12822: f64, t2612: f64, t12767: f64, t30630: f64, t10629: f64, t3407: f64, t1017: f64, t40558: f64, t7703: f64) -> (f64, f64, f64, f64, f64) {
    let t47297 = 16.0_f64 / 9.0_f64 * t1820 * t2559 * t30455 * t3342;
    let t47299 = 32.0_f64 / 15.0_f64 * t2612 * t12822;
    let t47301 = 32.0_f64 / 15.0_f64 * t30630 * t12767;
    let t47303 = 32.0_f64 / 15.0_f64 * t10629 * t3407;
    let t47307 = 32.0_f64 / 5.0_f64 * t1820 * t7703 * t40558 * t1017;
    (t47297, t47299, t47301, t47303, t47307)
}
