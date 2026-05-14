//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 640/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk640<F: Float>(t678: F, t9090: F, t1540: F, t687: F, t2144: F, t5267: F, t1971: F, t3351: F, t2376: F, t2604: F, t333: F, t618: F, t511: F, t7231: F, t352: F, t515: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9091 = t9090 * t678;
    let t9093 = t1540 * t687;
    let t9095 = t2144 * t5267;
    let t9096 = t1971 * t9095;
    let t9097 = t3351 * t9096;
    let t9102 = t2604 * t2376;
    let t9104 = t618 * t333;
    let t9105 = t511 * t9104;
    let t9106 = t7231 * t9105;
    let t9107 = t3351 * t9106;
    let t9109 = t618 * t352;
    let t9110 = t515 * t9109;
    (t9091, t9093, t9096, t9097, t9102, t9104, t9106, t9107, t9109, t9110)
}
