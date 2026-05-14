//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 810/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk810<F: Float>(t5275: F, t579: F, t16662: F, t16665: F, t16668: F, t16756: F, t16758: F, t16760: F, t16765: F, t16768: F, t16771: F, t16775: F, t5322: F, t633: F, t5175: F, t590: F) -> (F, F, F, F) {
    let t16777 = 8.0 / 15.0 * t579 * t5275;
    let t16778 = t16662 + t16665 - t16668 + t16756 - t16758 + t16760 + t16765 - t16768 - t16771 - t16775 - t16777;
    let t16780 = t633 * t5322;
    let t16781 = 16.0 / 15.0 * t16780;
    let t16782 = t590 * t5175;
    (t16777, t16778, t16781, t16782)
}
