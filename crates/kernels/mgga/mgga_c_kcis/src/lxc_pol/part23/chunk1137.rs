//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1137/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1137<F: Float>(t27348: F, t8144: F, t28397: F, t303: F, t4109: F, t8175: F, t28339: F, t3728: F, t27369: F, t28353: F, t28461: F, t7908: F, t7916: F, t94208: F, t94656: F, t94662: F, t98141: F, t98190: F, t98286: F) -> (F, F, F) {
    let t98909 = 0.46336805555555555556e-3 * t8144 * t27348;
    let t98911 = 0.61836467013888888889e-4 * t28397 * t27348;
    let t98915 = t303 * t4109 * t8175;
    let t98918 = t3728 * t28339;
    let t98929 = t98909 + t98911 + 0.13901041666666666667e-2 * t28461 * t7916 - 0.66327777777777777776e-2 * t98915 + 0.33163888888888888888e-2 * t94656 - 0.22109259259259259258e-2 * t98918 - 0.2782641015625e-3 * t27369 * t98286 - 0.22109259259259259258e-2 * t94662 - 0.69505208333333333333e-3 * t7908 * t98141 - 0.556528203125e-3 * t27369 * t98190 - 0.556528203125e-3 * t94208 * t28353;
    (t98915, t98918, t98929)
}
