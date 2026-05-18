//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 477/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk477<F: Float>(t398: F, t4374: F, t3532: F, t539: F, t1588: F, t442: F, t1390: F, t397: F, t3979: F, t535: F, t24: F) -> (F, F, F, F, F, F, F) {
    let t4375 = t398 * t4374;
    let t4391 = t539 * t3532;
    let t4400 = t1588 * t442;
    let t4406 = t539 * t1390;
    let t4416 = t397 * t3979 * t539;
    let t4418 = F::new(0.59969295720591057378e-2) * t535 * t4416;
    let t4419 = t397 * t24;
    (t4375, t4391, t4400, t4406, t4416, t4418, t4419)
}
