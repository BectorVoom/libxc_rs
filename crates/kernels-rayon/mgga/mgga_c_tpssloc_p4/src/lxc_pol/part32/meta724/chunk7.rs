//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2325/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325(t27628: f64, t95648: f64, t104118: f64, t24682: f64, t460: f64, t104122: f64, t27635: f64, t3: f64, t95326: f64, t11716: f64, t1210: f64, t1215: f64, t24685: f64, t27636: f64, t27638: f64, t27639: f64, t27644: f64, t27645: f64, t29594: f64, t29644: f64, t29648: f64, t3503: f64, t6218: f64, t6224: f64, t7331: f64, t8040: f64, t85966: f64, t86234: f64, t95396: f64, t95415: f64, t95649: f64) -> f64 {
    let t104231 = t95648 * t27628;
    let t104235 = t24682 * t104118 * t460;
    let t104239 = t24682 * t104122 * t460;
    let t104257 = t95326 * t3 * t27635;
    let t104264 = -0.20186378047070195428e-3_f64 * t86234 * t29644 + 0.60559134141210586284e-3_f64 * t95396 * t11716 * t6224 * t85966 * t1215 + 0.16149102437656156342e-2_f64 * t104231 * t7331 + 0.20186378047070195428e-3_f64 * t104235 * t7331 - 0.10093189023535097714e-3_f64 * t104239 * t7331 + 0.10093189023535097714e-3_f64 * t86234 * t29648 - 0.10093189023535097714e-3_f64 * t24685 * t29594 + 0.20186378047070195428e-3_f64 * t27636 * t3503 * t6218 * t27638 - 0.10093189023535097714e-3_f64 * t27636 * t1210 * t6218 * t27644 + 0.16149102437656156342e-2_f64 * t95649 * t8040 - 0.32298204875312312684e-2_f64 * t104257 * t27639 + 0.16149102437656156342e-2_f64 * t104257 * t27645 + 0.20186378047070195428e-3_f64 * t95415 * t8040;
    t104264
}
