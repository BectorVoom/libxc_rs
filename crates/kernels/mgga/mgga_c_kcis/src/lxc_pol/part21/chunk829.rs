//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 829/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk829<F: Float>(t10470: F, t361: F, t1127: F, t3245: F, t2822: F, t2852: F, t1141: F, t3323: F, t1138: F, t3329: F) -> (F, F, F, F, F, F) {
    let t10471 = t10470 * t361;
    let t10472 = F::cast_from(0.73697530864197530862e-3_f64) * t10471;
    let t10473 = t3245 * t1127;
    let t10477 = t2822 * t2852;
    let t10488 = t3323 * t1141;
    let t10491 = t1138 * t3329;
    (t10471, t10472, t10473, t10477, t10488, t10491)
}
