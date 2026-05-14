//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 655/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk655<F: Float>(t5470: F, t645: F, t1627: F, t1635: F, t1645: F, t1630: F, t1634: F, t639: F, t1639: F, t9: F, t1644: F, t4373: F, t643: F, t642: F, t1724: F, t1791: F, t661: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5472 = 4.0 / 15.0 * t5470 * t645;
    let t5474 = 4.0 / 15.0 * t1627 * t1635;
    let t5476 = 4.0 / 9.0 * t1627 * t1645;
    let t5477 = t1630 * t1634;
    let t5478 = t639 * t5477;
    let t5479 = 8.0 / 45.0 * t5478;
    let t5480 = t9 * t1639;
    let t5481 = t5480 * t1644;
    let t5482 = t639 * t5481;
    let t5483 = 8.0 / 27.0 * t5482;
    let t5484 = t643 * t4373;
    let t5485 = t642 * t5484;
    let t5487 = 4.0 / 45.0 * t639 * t5485;
    let t5489 = t1791 * t661 * t1724;
    (t5472, t5474, t5476, t5477, t5479, t5480, t5481, t5483, t5484, t5485, t5487, t5489)
}
