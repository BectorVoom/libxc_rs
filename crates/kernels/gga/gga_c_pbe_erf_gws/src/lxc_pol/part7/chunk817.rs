//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk817<F: Float>(t6732: F, t6733: F, t6735: F, t6736: F, t339: F, t338: F, t376: F, t2271: F, t2365: F, t822: F, t833: F, t2367: F, t2397: F) -> (F, F, F, F, F, F, F) {
    let t6738 = t6732 + t6733 + t6735 + t6736;
    let t6739 = t339 * t6738;
    let t6741 = t338 * t6739 * t376;
    let t6744 = t2271 * t2365;
    let t6745 = t822 * t6744;
    let t6746 = t6745 * t833;
    let t6748 = t2367 * t2397;
    (t6738, t6739, t6741, t6744, t6745, t6746, t6748)
}
