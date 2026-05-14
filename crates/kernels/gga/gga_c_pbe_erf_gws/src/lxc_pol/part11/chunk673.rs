//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 673/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk673<F: Float>(t3440: F, t401: F, t3434: F, t3437: F, t3342: F, t4957: F, t4951: F, t3422: F, t395: F, t3426: F, t3430: F, t3584: F, t723: F, t3398: F, t586: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10756 = t401 * t3440;
    let t10758 = t401 * t3434;
    let t10760 = t401 * t3437;
    let t10777 = t4957 * t3342;
    let t10783 = t4951 * t3342;
    let t10823 = t395 * t3422;
    let t10825 = t395 * t3426;
    let t10827 = t395 * t3430;
    let t10841 = t3584 * t723;
    let t10843 = t3398 * t586;
    (t10756, t10758, t10760, t10777, t10783, t10823, t10825, t10827, t10841, t10843)
}
