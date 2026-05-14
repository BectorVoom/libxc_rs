//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 659/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk659<F: Float>(t199: F, t5516: F, t266: F, t331: F, t265: F, t1640: F, t649: F, t1692: F, t661: F, t639: F, t1824: F, t5312: F, t1769: F, t610: F, t1827: F, t587: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5518 = 4.0 / 5.0 * t5516 * t199;
    let t5519 = t266 * t331;
    let t5521 = 8.0 / 405.0 * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5523 = t1692 * t661;
    let t5524 = t5522 * t5523;
    let t5526 = 4.0 / 9.0 * t639 * t5524;
    let t5528 = 16.0 / 15.0 * t5312 * t1824;
    let t5529 = t1769 * t610;
    let t5530 = t1827 * t5529;
    let t5532 = 4.0 / 15.0 * t587 * t5530;
    (t5518, t5519, t5521, t5522, t5523, t5524, t5526, t5528, t5529, t5530, t5532)
}
