//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 723/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk723<F: Float>(t2005: F, t5483: F, t1849: F, t2020: F, t2023: F, t3290: F, t1775: F, t1060: F, t5515: F, t5491: F, t10791: F, t397: F, t786: F, t782: F, t2009: F, t5465: F) -> (F, F, F, F, F, F) {
    let t12230 = t2005 * t5483;
    let t12234 = t2020 * t1849;
    let t12235 = t3290 * t2023;
    let t12236 = t12234 * t12235;
    let t12237 = t1775 * t12236;
    let t12240 = t1060 * t5515;
    let t12241 = t5491 * t12240;
    let t12242 = t1775 * t12241;
    let t12246 = t397 * t10791 * t786;
    let t12248 = 0.9994882620098509563e-2 * t782 * t12246;
    let t12249 = t5465 * t2009;
    (t12230, t12235, t12237, t12242, t12248, t12249)
}
