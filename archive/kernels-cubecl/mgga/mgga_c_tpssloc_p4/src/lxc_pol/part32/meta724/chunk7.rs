//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2325/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325<F: Float>(t27628: F, t95648: F, t104118: F, t24682: F, t460: F, t104122: F, t27635: F, t3: F, t95326: F, t11716: F, t1210: F, t1215: F, t24685: F, t27636: F, t27638: F, t27639: F, t27644: F, t27645: F, t29594: F, t29644: F, t29648: F, t3503: F, t6218: F, t6224: F, t7331: F, t8040: F, t85966: F, t86234: F, t95396: F, t95415: F, t95649: F) -> F {
    let t104231 = t95648 * t27628;
    let t104235 = t24682 * t104118 * t460;
    let t104239 = t24682 * t104122 * t460;
    let t104257 = t95326 * t3 * t27635;
    let t104264 = -F::cast_from(0.20186378047070195428e-3_f64) * t86234 * t29644 + F::cast_from(0.60559134141210586284e-3_f64) * t95396 * t11716 * t6224 * t85966 * t1215 + F::cast_from(0.16149102437656156342e-2_f64) * t104231 * t7331 + F::cast_from(0.20186378047070195428e-3_f64) * t104235 * t7331 - F::cast_from(0.10093189023535097714e-3_f64) * t104239 * t7331 + F::cast_from(0.10093189023535097714e-3_f64) * t86234 * t29648 - F::cast_from(0.10093189023535097714e-3_f64) * t24685 * t29594 + F::cast_from(0.20186378047070195428e-3_f64) * t27636 * t3503 * t6218 * t27638 - F::cast_from(0.10093189023535097714e-3_f64) * t27636 * t1210 * t6218 * t27644 + F::cast_from(0.16149102437656156342e-2_f64) * t95649 * t8040 - F::cast_from(0.32298204875312312684e-2_f64) * t104257 * t27639 + F::cast_from(0.16149102437656156342e-2_f64) * t104257 * t27645 + F::cast_from(0.20186378047070195428e-3_f64) * t95415 * t8040;
    t104264
}
