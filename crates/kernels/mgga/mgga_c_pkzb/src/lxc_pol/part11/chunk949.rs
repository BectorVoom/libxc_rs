//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 949/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk949<F: Float>(t11232: F, t11314: F, t237: F, t11164: F, t10182: F, t3139: F, t898: F, t11180: F, t6230: F, t6233: F, t6121: F, t2320: F, t398: F, t19: F, t297: F, t326: F, t397: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11316 = t237 * (t11232 + t11314);
    let t11318 = 0.19751673498613801407e-1 * t237 * t11164;
    let t11319 = t10182 * t3139;
    let t11321 = 0.51947577317044391277e2 * t898 * t11319;
    let t11322 = t6230 * t11180;
    let t11323 = t11322 * t6233;
    let t11325 = 0.10254018858216406658e4 * t898 * t11323;
    let t11326 = t6121 * t11180;
    let t11327 = t11326 * t2320;
    let t11329 = 0.10389515463408878255e3 * t898 * t11327;
    let t11333 = t398 * t398;
    let t11335 = 1.0 / t19 / t11333;
    let t11338 = t397 * t326 * t11335 * t297;
    (t11316, t11318, t11319, t11321, t11322, t11323, t11325, t11326, t11327, t11329, t11335, t11338)
}
