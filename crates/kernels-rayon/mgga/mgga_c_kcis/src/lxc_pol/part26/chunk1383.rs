//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1383/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1383(t28426: f64, t8144: f64, t102725: f64, t102729: f64, t102731: f64, t102740: f64, t102743: f64, t102746: f64, t28480: f64, t29404: f64, t7901: f64, t8159: f64, t98909: f64, t98911: f64, t98918: f64) -> f64 {
    let t103749 = t8144 * t28426;
    let t103762 = 0.46336805555555555557e-3_f64 * t103749 + t98909 + t98911 + 0.33163888888888888888e-2_f64 * t102725 - 0.33163888888888888888e-2_f64 * t102729 - 0.36848765432098765431e-3_f64 * t102731 + 0.67960648148148148147e-2_f64 * t29404 * t7901 - 0.37069444444444444444e-2_f64 * t28480 * t8159 - 0.22109259259259259259e-2_f64 * t98918 + 0.13265555555555555555e-1_f64 * t102740 - 0.13265555555555555555e-1_f64 * t102743 + 0.24320185185185185185e-1_f64 * t102746;
    t103762
}
