//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1116/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1116<F: Float>(t12246: F, t2239: F, t27459: F, t28344: F, t28353: F, t28373: F, t4001: F, t52696: F, t6140: F, t7908: F, t98010: F, t98190: F, t98304: F, t98445: F, t98449: F, t98452: F, t98458: F) -> (F,) {
    let t98460 = 0.30891203703703703704e-3 * t7908 * t98010 + 0.41703125000000000001e-2 * t7908 * t52696 * t28373 * t4001 - 0.27802083333333333334e-2 * t27459 * t28353 - 0.27802083333333333334e-2 * t7908 * t98190 - 0.13901041666666666667e-2 * t27459 * t28344 - 0.13901041666666666667e-2 * t7908 * t98304 - 0.69505208333333333333e-3 * t7908 * t98445 + 0.16581944444444444444e-2 * t98449 - 0.66327777777777777776e-2 * t98452 + 0.18534722222222222222e-2 * t12246 * t6140 * t2239 + 0.73697530864197530861e-2 * t98458;
    (t98460,)
}
