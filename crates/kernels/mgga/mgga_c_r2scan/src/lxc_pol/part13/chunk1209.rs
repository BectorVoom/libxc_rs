//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1209/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1209<F: Float>(t11498: F, t37282: F, t3446: F, t3453: F, t7104: F, t10655: F, t11603: F, t10922: F, t11572: F, t3308: F, t3429: F, t7136: F) -> (F, F, F, F, F, F) {
    let t40509 = F::new(3.0) / F::new(2.0) * t37282 * t11498;
    let t40511 = t3446 * t3453 * t7104;
    let t40513 = t10655 * t11603;
    let t40515 = t10922 * t11603;
    let t40518 = t3429 * t3308 * t11572;
    let t40519 = F::new(0.30487649791575028314e-3) * t40518;
    let t40521 = t3446 * t3453 * t7136;
    (t40509, t40511, t40513, t40515, t40519, t40521)
}
