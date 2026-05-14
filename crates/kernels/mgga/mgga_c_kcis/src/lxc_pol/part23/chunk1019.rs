//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1019/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1019<F: Float>(t4314: F, t4455: F, t779: F, t9274: F, t2531: F, t2537: F, t782: F, t9266: F, t142: F, t164: F, t9273: F, t113: F, t8750: F, t898: F, t9005: F, t2764: F, t2770: F) -> (F, F, F, F, F, F, F, F) {
    let t30424 = t4455 * t4314;
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    let t35630 = t9266 * t782;
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    let t36429 = t9005 * t898;
    let t36436 = t2764 * t2770;
    (t30424, t31271, t31274, t35630, t35635, t36222, t36429, t36436)
}
