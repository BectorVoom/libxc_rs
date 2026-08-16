//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1280/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1280(t27348: f64, t8144: f64, t28397: f64, t303: f64, t4109: f64, t8175: f64, t28339: f64, t3728: f64, t27369: f64, t28353: f64, t28461: f64, t7908: f64, t7916: f64, t94208: f64, t94656: f64, t94662: f64, t98141: f64, t98190: f64, t98286: f64) -> (f64, f64, f64) {
    let t98909 = 0.46336805555555555556e-3_f64 * t8144 * t27348;
    let t98911 = 0.61836467013888888889e-4_f64 * t28397 * t27348;
    let t98915 = t303 * t4109 * t8175;
    let t98918 = t3728 * t28339;
    let t98929 = t98909 + t98911 + 0.13901041666666666667e-2_f64 * t28461 * t7916 - 0.66327777777777777776e-2_f64 * t98915 + 0.33163888888888888888e-2_f64 * t94656 - 0.22109259259259259258e-2_f64 * t98918 - 0.2782641015625e-3_f64 * t27369 * t98286 - 0.22109259259259259258e-2_f64 * t94662 - 0.69505208333333333333e-3_f64 * t7908 * t98141 - 0.556528203125e-3_f64 * t27369 * t98190 - 0.556528203125e-3_f64 * t94208 * t28353;
    (t98915, t98918, t98929)
}
