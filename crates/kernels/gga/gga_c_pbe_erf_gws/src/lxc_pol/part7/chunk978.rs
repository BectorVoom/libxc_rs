//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 978/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk978<F: Float>(t2100: F, t353: F, t859: F, t898: F, t938: F, t2074: F, t4386: F, t11374: F, t822: F, t6161: F, t376: F, t810: F, t2352: F, t6781: F, t829: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t19599 = t859 * t353 * t898 * t2100 * t938;
    let t19602 = t898 * t2074;
    let t19605 = t4386 * t353 * t19602 * t938;
    let t19608 = t822 * t11374;
    let t19612 = t859 * t353 * t898 * t6161 * t938;
    let t19615 = t376 * t2100;
    let t19618 = t4386 * t353 * t19615 * t810;
    let t19621 = t6781 * t2352;
    let t19623 = t829 * t830 * t19621;
    (t19599, t19602, t19605, t19608, t19612, t19615, t19618, t19623)
}
