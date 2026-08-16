//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1155/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1155<F: Float>(t26891: F, t8069: F, t5091: F, t7748: F, t28041: F, t28043: F, t28046: F, t28048: F, t28051: F, t28053: F, t28055: F, t28057: F, t28060: F, t28062: F, t28064: F) -> (F, F, F) {
    let t28066 = t26891 * t8069;
    let t28068 = t7748 * t5091;
    let t28070 = t28041 / F::cast_from(96.0_f64) + t28043 / F::cast_from(8.0_f64) + t28046 / F::cast_from(24.0_f64) - t28048 / F::cast_from(96.0_f64) - t28051 / F::cast_from(16.0_f64) - t28053 / F::cast_from(16.0_f64) + t28055 / F::cast_from(24.0_f64) - t28057 / F::cast_from(9.0_f64) - t28060 / F::cast_from(16.0_f64) + t28062 / F::cast_from(128.0_f64) + t28064 / F::cast_from(128.0_f64) + t28066 / F::cast_from(6.0_f64) - t28068 / F::cast_from(24.0_f64);
    (t28066, t28068, t28070)
}
