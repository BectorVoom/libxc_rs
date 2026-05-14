//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 512/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk512<F: Float>(t1037: F, t2612: F, t3354: F, t643: F, t642: F, t639: F, t1643: F, t3351: F, t1640: F, t3346: F, t591: F, t590: F, t587: F, t1664: F, t3342: F, t1661: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3517 = 8.0 / 45.0 * t2612 * t1037;
    let t3518 = t643 * t3354;
    let t3519 = t642 * t3518;
    let t3521 = 4.0 / 45.0 * t639 * t3519;
    let t3522 = t1643 * t3351;
    let t3523 = t1640 * t3522;
    let t3525 = 4.0 / 27.0 * t639 * t3523;
    let t3526 = t591 * t3346;
    let t3527 = t590 * t3526;
    let t3529 = 4.0 / 45.0 * t587 * t3527;
    let t3530 = t1664 * t3342;
    let t3531 = t1661 * t3530;
    (t3517, t3518, t3519, t3521, t3522, t3523, t3525, t3526, t3527, t3529, t3530, t3531)
}
