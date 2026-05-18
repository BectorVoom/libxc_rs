//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1108/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1108<F: Float>(t481: F, t9577: F, t792: F, t1234: F, t3574: F, t2259: F, t2867: F, t10943: F, t11603: F, t2333: F, t2847: F, t795: F) -> (F, F, F, F, F, F) {
    let t39264 = t9577 * t481;
    let t39268 = t9577 * t792;
    let t39279 = t3574 * t1234;
    let t39286 = t2867 * t2259;
    let t39290 = t10943 * t11603;
    let t39299 = t2333 * t2847;
    let t39300 = t39299 * t795;
    (t39264, t39268, t39279, t39286, t39290, t39300)
}
