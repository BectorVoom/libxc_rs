//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 519/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk519<F: Float>(t2157: F, t892: F, t132: F, t1338: F, t2039: F, t638: F, t303: F, t31: F, t2046: F, t2050: F, t357: F, t668: F, t934: F, t1990: F, t2186: F, t4443: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7383 = t892 * t2157;
    let t7384 = 0.79828278012425390426e-1 * t7383;
    let t7385 = t132 * t1338;
    let t7387 = t638 * t2039 * t7385;
    let t7389 = t303 * t31;
    let t7391 = t2046 * t2050 * t7389;
    let t7392 = 0.43368970657079495312e-4 * t7391;
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7396 = 0.43368970657079495312e-4 * t7395;
    let t7399 = t934 * t668;
    let t7402 = t2186 * t1990;
    let t7403 = 0.19863479950205658386e-4 * t7402;
    let t7407 = t671 * t4443;
    (t7384, t7385, t7387, t7389, t7392, t7393, t7396, t7399, t7403, t7407)
}
