//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 601/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk601<F: Float>(t10350: F, t1152: F, t4570: F, t3474: F, t1636: F, t5294: F, t5184: F, t5182: F, t5302: F, t5192: F, t5060: F, t654: F, t5285: F, t1894: F, t3290: F, t6675: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t10351 = 6.0 * t10350;
    let t10352 = t1152 * t4570;
    let t10353 = 3.0 / 16.0 * t10352;
    let t10354 = t1152 * t3474;
    let t10355 = 3.0 / 16.0 * t10354;
    let t10356 = t5294 * t1636;
    let t10357 = t5184 * t10356;
    let t10358 = t5182 * t10357;
    let t10360 = t5302 * t1636;
    let t10361 = t5192 * t10360;
    let t10362 = t5182 * t10361;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    let t10366 = t5285 * t1636;
    let t10367 = t10365 * t10366;
    let t10368 = t5182 * t10367;
    let t10370 = t3290 * t1894;
    let t10371 = t6675 * t10370;
    (t10351, t10353, t10355, t10358, t10362, t10368, t10370, t10371)
}
