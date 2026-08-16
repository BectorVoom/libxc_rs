//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1252/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1252(t27364: f64, t28331: f64, t5780: f64, t11806: f64, t16695: f64, t7923: f64, t12246: f64, t2239: f64, t27459: f64, t28344: f64, t28353: f64, t28373: f64, t4001: f64, t52696: f64, t6140: f64, t7908: f64, t98010: f64, t98190: f64, t98304: f64, t98445: f64, t98449: f64) -> (f64, f64, f64) {
    let t98452 = t5780 * t27364 * t28331;
    let t98458 = t11806 * t7923 * t16695;
    let t98460 = 0.30891203703703703704e-3_f64 * t7908 * t98010 + 0.41703125000000000001e-2_f64 * t7908 * t52696 * t28373 * t4001 - 0.27802083333333333334e-2_f64 * t27459 * t28353 - 0.27802083333333333334e-2_f64 * t7908 * t98190 - 0.13901041666666666667e-2_f64 * t27459 * t28344 - 0.13901041666666666667e-2_f64 * t7908 * t98304 - 0.69505208333333333333e-3_f64 * t7908 * t98445 + 0.16581944444444444444e-2_f64 * t98449 - 0.66327777777777777776e-2_f64 * t98452 + 0.18534722222222222222e-2_f64 * t12246 * t6140 * t2239 + 0.73697530864197530861e-2_f64 * t98458;
    (t98452, t98458, t98460)
}
