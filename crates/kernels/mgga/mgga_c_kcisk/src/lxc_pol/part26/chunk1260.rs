//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1260/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1260<F: Float>(t31932: F, t9301: F, t31944: F, t9311: F, t9315: F, t31910: F, t31952: F, t31928: F, t9320: F, t31917: F, t15208: F, t9314: F, t111326: F, t31960: F, t31905: F, t110950: F, t111340: F) -> (F,) {
    let t111342 = t9301 * t31932;
    let t111344 = t9311 * t31944;
    let t111347 = t9315 * t31944;
    let t111349 = t31952 * t31910;
    let t111351 = t9315 * t31932;
    let t111353 = t31928 * t9320;
    let t111355 = t9311 * t31917;
    let t111358 = t15208 * t9314 * t31910;
    let t111360 = t31960 * t111326;
    let t111362 = t31905 * t31932;
    let t111364 = 0.24125000000000000001e-1 * t111340 + 0.31250000000000000001e-1 * t111342 + 0.31250000000000000001e-1 * t111344 - 0.29847499999999999999e-1 * t110950 - 0.72916666666666666668e-1 * t111347 - 0.36187500000000000001e-1 * t111349 - 0.72916666666666666668e-1 * t111351 - 0.14583333333333333334e0 * t111353 - 0.62500000000000000002e-1 * t111355 + 0.16296437500000000001e-1 * t111358 + 0.13968375e-1 * t111360 - 0.28145833333333333334e-1 * t111362;
    (t111364,)
}
