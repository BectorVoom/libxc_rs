//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 682/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk682(t5470: f64, t645: f64, t1627: f64, t1635: f64, t1645: f64, t1630: f64, t1634: f64, t639: f64, t1639: f64, t9: f64, t1644: f64, t4373: f64, t643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5472 = 4.0_f64 / 15.0_f64 * t5470 * t645;
    let t5474 = 4.0_f64 / 15.0_f64 * t1627 * t1635;
    let t5476 = 4.0_f64 / 9.0_f64 * t1627 * t1645;
    let t5477 = t1630 * t1634;
    let t5478 = t639 * t5477;
    let t5479 = 8.0_f64 / 45.0_f64 * t5478;
    let t5480 = t9 * t1639;
    let t5481 = t5480 * t1644;
    let t5482 = t639 * t5481;
    let t5483 = 8.0_f64 / 27.0_f64 * t5482;
    let t5484 = t643 * t4373;
    (t5472, t5474, t5476, t5477, t5479, t5480, t5481, t5483, t5484)
}
