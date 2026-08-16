//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 687/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk687(t5522: f64, t5523: f64, t639: f64, t1824: f64, t5312: f64, t1769: f64, t610: f64, t1827: f64, t587: f64, t1821: f64, t4972: f64, t2559: f64, t4963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5524 = t5522 * t5523;
    let t5526 = 4.0_f64 / 9.0_f64 * t639 * t5524;
    let t5528 = 16.0_f64 / 15.0_f64 * t5312 * t1824;
    let t5529 = t1769 * t610;
    let t5530 = t1827 * t5529;
    let t5532 = 4.0_f64 / 15.0_f64 * t587 * t5530;
    let t5533 = t1821 * t4972;
    let t5535 = 8.0_f64 / 15.0_f64 * t587 * t5533;
    let t5536 = t2559 * t4963;
    (t5524, t5526, t5528, t5529, t5530, t5532, t5533, t5535, t5536)
}
