//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 546/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk546<F: Float>(t2900: F, t2923: F, t302: F, t1066: F, t759: F, t761: F, t2105: F, t179: F, t2068: F, t299: F, t197: F, t290: F, t294: F, t297: F, t46: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2924 = t2900 * t2923;
    let t2925 = t302 * t2924;
    let t2931 = t1066 * t759;
    let t2932 = t2931 * t761;
    let t2933 = t2105 * t2932;
    let t2939 = t179 * t2068 * t1066;
    let t2940 = t299 * t2939;
    let t2942 = t290 * t197;
    let t2944 = t294 * t297 * t46;
    let t2945 = t2942 * t2944;
    (t2924, t2925, t2931, t2932, t2933, t2939, t2940, t2942, t2945)
}
