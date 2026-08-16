//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1155/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1155(t26891: f64, t8069: f64, t5091: f64, t7748: f64, t28041: f64, t28043: f64, t28046: f64, t28048: f64, t28051: f64, t28053: f64, t28055: f64, t28057: f64, t28060: f64, t28062: f64, t28064: f64) -> (f64, f64, f64) {
    let t28066 = t26891 * t8069;
    let t28068 = t7748 * t5091;
    let t28070 = t28041 / 96.0_f64 + t28043 / 8.0_f64 + t28046 / 24.0_f64 - t28048 / 96.0_f64 - t28051 / 16.0_f64 - t28053 / 16.0_f64 + t28055 / 24.0_f64 - t28057 / 9.0_f64 - t28060 / 16.0_f64 + t28062 / 128.0_f64 + t28064 / 128.0_f64 + t28066 / 6.0_f64 - t28068 / 24.0_f64;
    (t28066, t28068, t28070)
}
