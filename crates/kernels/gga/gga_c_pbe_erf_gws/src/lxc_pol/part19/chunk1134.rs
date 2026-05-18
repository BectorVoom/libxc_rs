//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1134/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1134<F: Float>(t3279: F, t4049: F, t14011: F, t3232: F, t1125: F, t14024: F, t3139: F, t9026: F, t4028: F, t14007: F, t3261: F, t3291: F) -> (F, F, F, F, F, F, F) {
    let t14516 = t4049 * t3279;
    let t14518 = t14011 * t3232;
    let t14520 = t1125 * t14024;
    let t14522 = t3139 * t9026;
    let t14523 = t4028 * t14522;
    let t14525 = t14007 * t3261;
    let t14529 = t14007 * t3291;
    (t14516, t14518, t14520, t14522, t14523, t14525, t14529)
}
