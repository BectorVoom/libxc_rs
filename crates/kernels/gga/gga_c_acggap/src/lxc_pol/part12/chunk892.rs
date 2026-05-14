//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 892/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk892<F: Float>(t31699: F, t8665: F, t30409: F, t30418: F, t31309: F, t525: F, t2016: F, t8622: F, t515: F, t7852: F, t2041: F, t4769: F, t2294: F, t7630: F, t31253: F, t527: F) -> (F, F, F, F, F, F, F) {
    let t33853 = t31699 * t8665;
    let t33857 = t31309 * t30418 * t30409 * t525;
    let t33859 = t2016 * t8622;
    let t33861 = t7852 * t515;
    let t33863 = t2041 * t4769;
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    (t33853, t33857, t33859, t33861, t33863, t33865, t33867)
}
