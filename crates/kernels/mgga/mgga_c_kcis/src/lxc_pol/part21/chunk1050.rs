//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1050/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1050<F: Float>(t11182: F, t1236: F, t3609: F, t3643: F, t11228: F, t433: F, t436: F, t782: F, t9266: F, t142: F, t164: F, t9273: F, t113: F, t8750: F, t898: F, t9005: F) -> (F, F, F, F, F, F, F) {
    let t35547 = t1236 * t11182;
    let t35576 = t3609 * t3643;
    let t35615 = t433 / t11228 / t436;
    let t35630 = t9266 * t782;
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    let t36429 = t9005 * t898;
    (t35547, t35576, t35615, t35630, t35635, t36222, t36429)
}
