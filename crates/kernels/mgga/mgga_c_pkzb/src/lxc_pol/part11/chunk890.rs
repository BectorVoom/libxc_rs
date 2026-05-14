//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 890/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk890<F: Float>(t3337: F, t995: F, t3356: F, t987: F, t3314: F, t973: F, t4794: F, t2489: F, t3318: F, t1429: F, t4803: F, t15: F, t20: F, t399: F, t3329: F, t983: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10408 = t3337 * t995;
    let t10411 = t987 * t3356;
    let t10414 = t3314 * t973;
    let t10415 = t4794 * t10414;
    let t10418 = t2489 * t3318;
    let t10421 = -t1429 - t4803;
    let t10422 = 3.0 * t10421;
    let t10423 = t15 * t10422;
    let t10427 = 1.0 / t20 / t399;
    let t10428 = sigma2 * t10427;
    let t10437 = t3329 * t983;
    (t10408, t10411, t10414, t10415, t10418, t10421, t10422, t10423, t10428, t10437)
}
