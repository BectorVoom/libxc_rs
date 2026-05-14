//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1403/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1403<F: Float>(t10191: F, t2099: F, t918: F, t2023: F, t3857: F, t46: F, t2364: F, t2394: F, t3214: F, t8315: F, t3882: F, t5939: F, t10197: F, t2376: F, t179: F, t19155: F, t3757: F, t404: F) -> (F, F, F, F, F, F, F) {
    let t28283 = t918 * t2099 * t10191;
    let t28287 = t3857 * t2023;
    let t28288 = t28287 * t46;
    let t28289 = t2364 * t28288;
    let t28292 = t2394 * t28288;
    let t28295 = t3214 * t8315;
    let t28303 = t918 * t5939 * t3882;
    let t28305 = t10197 * t2376;
    let t28316 = t404 * t179 * t19155 * t3757;
    (t28283, t28289, t28292, t28295, t28303, t28305, t28316)
}
