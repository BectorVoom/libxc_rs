//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1093/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1093<F: Float>(t2200: F, t329: F, t369: F, t2404: F, t376: F, t6738: F, t829: F, t830: F, t2222: F, t2242: F, t2298: F, t332: F, t19631: F, t20808: F, t21011: F, t21106: F, t21762: F, t21764: F, t21768: F, t21771: F, t21775: F, t21777: F, t2220: F, t2353: F, t2383: F, t2387: F, t2408: F, t2409: F, t2410: F, t326: F, t335: F, t338: F, t353: F, t6159: F, t6164: F, t822: F, t826: F, t827: F, t833: F) -> (F,) {
    let t21780 = t329 * t2200 * t369;
    let t21781 = t21780 * t2404;
    let t21785 = t829 * t830 * t6738 * t376;
    let t21788 = t2242 * t2222;
    let t21807 = t329 * t332 * t2298;
    let t21813 = t2387 * t6159 * t6164 / 16.0 + 7.0 / 36.0 * t21762 - t822 * t21764 * t21768 / 16.0 + 7.0 / 72.0 * t21771 + 7.0 / 36.0 * t21775 + 7.0 / 12.0 * t21777 + 35.0 / 12.0 * t21781 + t827 * t21785 / 24.0 + 35.0 / 72.0 * t21788 + t326 * t20808 * t2383 * t833 / 32.0 + t326 * t21106 * t826 * t833 / 96.0 + t2408 * t2409 * t19631 * t2410 / 4.0 - t335 * t338 * t2220 * t2353 / 16.0 + 5.0 / 4.0 * t21807 * t338 * t353 * t376 * t21011;
    (t21813,)
}
