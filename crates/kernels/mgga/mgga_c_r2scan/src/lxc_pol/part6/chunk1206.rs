//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1206/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1206<F: Float>(t22228: F, t5375: F, t1731: F, t20: F, t5947: F, t4911: F, t726: F, t21234: F, t226: F, t5455: F, t5460: F, t721: F, t1982: F, t5530: F, t61: F, t21248: F) -> (F, F, F, F, F, F, F, F) {
    let t22229 = t5375 * t22228;
    let t22232 = t1731 * t20 * t5947;
    let t22233 = t5375 * t22232;
    let t22235 = t4911 * t726;
    let t22239 = 0.84214420165938905383e2 * t5455 * t226 * t21234;
    let t22242 = 0.37402255668271961718e4 * t5460 * t721 * t21234;
    let t22246 = 0.73828935779158127934e5 * t61 * t5530 * t1982 * t21234;
    let t22249 = 0.41016075432865626632e4 * t5460 * t1982 * t21248;
    (t22229, t22232, t22233, t22235, t22239, t22242, t22246, t22249)
}
