//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 546/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk546<F: Float>(t2157: F, t892: F, t132: F, t1338: F, t2039: F, t638: F, t303: F, t31: F, t2046: F, t2050: F, t357: F, t1990: F, t2186: F) -> (F, F, F, F, F, F, F, F) {
    let t7383 = t892 * t2157;
    let t7385 = t132 * t1338;
    let t7387 = t638 * t2039 * t7385;
    let t7389 = t303 * t31;
    let t7391 = t2046 * t2050 * t7389;
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7402 = t2186 * t1990;
    (t7383, t7385, t7387, t7389, t7391, t7393, t7395, t7402)
}
