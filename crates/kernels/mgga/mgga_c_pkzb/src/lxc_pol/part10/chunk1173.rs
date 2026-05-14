//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1173/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1173<F: Float>(t2185: F, t2370: F, t2411: F, t54: F, t2255: F, t2277: F, t356: F, t2288: F, t2295: F, t2317: F, t2249: F, t2278: F, t18439: F, t18442: F, t6141: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18649 = t2370 * t2185;
    let t18657 = t54 * t2411;
    let t18706 = t356 / t2277 / t2255;
    let t18711 = t2288 * t2295;
    let t18740 = t2288 * t2317;
    let t18747 = t2249 * t2278;
    let t18750 = 0.17757530864197530864e0 * t18439;
    let t18765 = 0.5356037037037037037e1 * t18439;
    let t18766 = 0.16979925925925925926e1 * t18442;
    let t18790 = t828 * t6141;
    (t18649, t18657, t18706, t18711, t18740, t18747, t18750, t18765, t18766, t18790)
}
