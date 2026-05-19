//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 578/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk578<F: Float>(t2393: F, t326: F, t2366: F, t2029: F, t394: F, t2369: F, t758: F, t405: F, t466: F, t178: F, t404: F, t53: F, t931: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2394 = t2393 * t326;
    let t2395 = t2394 * t2366;
    let t2396 = t2029 * t394;
    let t2397 = t2369 * t2396;
    let t2398 = t758 * t2397;
    let t2401 = t466 * t405;
    let t2402 = t178 * t2401;
    let t2404 = F::cast_from(0.47637797908966374413e-4_f64) * t404 * t2402;
    let t2405 = t53 * t931;
    (t2394, t2395, t2396, t2397, t2398, t2401, t2402, t2404, t2405)
}
