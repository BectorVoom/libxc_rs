//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1234/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1234<F: Float>(t21773: F, t2362: F, t822: F, t2373: F, t4453: F, t2200: F, t329: F, t369: F, t2404: F, t376: F, t6738: F, t829: F, t830: F) -> (F, F, F, F) {
    let t21775 = t822 * t21773 * t2362;
    let t21777 = t4453 * t2373;
    let t21780 = t329 * t2200 * t369;
    let t21781 = t21780 * t2404;
    let t21785 = t829 * t830 * t6738 * t376;
    (t21775, t21777, t21781, t21785)
}
