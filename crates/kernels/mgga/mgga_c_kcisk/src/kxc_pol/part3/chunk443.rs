//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 443/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk443<F: Float>(t382: F, t442: F, t1056: F, t1286: F, t3484: F, t3482: F, t1216: F, t1219: F, t1299: F, t1340: F, t1411: F, t143: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3485 = t382 * t442;
    let t3486 = t1056 * t1286;
    let t3487 = t3485 * t3486;
    let t3488 = t3484 * t3487;
    let t3489 = t3482 * t3488;
    let t3491 = t1216 * t1219;
    let t3494 = t1299 * t382;
    let t3495 = t3494 * t1286;
    let t3496 = t1340 * t3495;
    let t3497 = t1411 * t3496;
    let t3499 = F::cast_from(2.0_f64) * t143;
    (t3485, t3487, t3488, t3489, t3491, t3494, t3495, t3496, t3497, t3499)
}
