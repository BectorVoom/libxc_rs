//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1252/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1252<F: Float>(t27364: F, t28331: F, t5780: F, t11806: F, t16695: F, t7923: F, t12246: F, t2239: F, t27459: F, t28344: F, t28353: F, t28373: F, t4001: F, t52696: F, t6140: F, t7908: F, t98010: F, t98190: F, t98304: F, t98445: F, t98449: F) -> (F, F, F) {
    let t98452 = t5780 * t27364 * t28331;
    let t98458 = t11806 * t7923 * t16695;
    let t98460 = F::cast_from(0.30891203703703703704e-3_f64) * t7908 * t98010 + F::cast_from(0.41703125000000000001e-2_f64) * t7908 * t52696 * t28373 * t4001 - F::cast_from(0.27802083333333333334e-2_f64) * t27459 * t28353 - F::cast_from(0.27802083333333333334e-2_f64) * t7908 * t98190 - F::cast_from(0.13901041666666666667e-2_f64) * t27459 * t28344 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t98304 - F::cast_from(0.69505208333333333333e-3_f64) * t7908 * t98445 + F::cast_from(0.16581944444444444444e-2_f64) * t98449 - F::cast_from(0.66327777777777777776e-2_f64) * t98452 + F::cast_from(0.18534722222222222222e-2_f64) * t12246 * t6140 * t2239 + F::cast_from(0.73697530864197530861e-2_f64) * t98458;
    (t98452, t98458, t98460)
}
