//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 954/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk954<F: Float>(t3440: F, t401: F, t3434: F, t3437: F, t572: F, t9788: F, t606: F, t10438: F, t10443: F, t1856: F, t3342: F, t4957: F) -> (F, F, F, F, F, F, F, F) {
    let t10756 = t401 * t3440;
    let t10758 = t401 * t3434;
    let t10760 = t401 * t3437;
    let t10762 = t572 * t9788;
    let t10763 = t606 * t10762;
    let t10771 = t606 * t10438;
    let t10774 = t1856 * t10443;
    let t10777 = t4957 * t3342;
    (t10756, t10758, t10760, t10762, t10763, t10771, t10774, t10777)
}
