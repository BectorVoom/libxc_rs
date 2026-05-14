//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 509/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk509<F: Float>(t1588: F, t442: F, t1056: F, t1591: F, t1312: F, t1390: F, t539: F, t3278: F, t1572: F, t1576: F, t397: F, t3979: F, t535: F, t24: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4400 = t1588 * t442;
    let t4401 = t1056 * t1591;
    let t4402 = t4400 * t4401;
    let t4403 = t1312 * t4402;
    let t4406 = t539 * t1390;
    let t4407 = t4406 * t3278;
    let t4408 = t1312 * t4407;
    let t4411 = t1572 * t1576;
    let t4416 = t397 * t3979 * t539;
    let t4418 = 0.59969295720591057378e-2 * t535 * t4416;
    let t4419 = t397 * t24;
    (t4400, t4402, t4403, t4406, t4407, t4408, t4411, t4416, t4418, t4419)
}
