//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1108/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1108<F: Float>(t39299: F, t795: F, t3275: F, t3276: F, t10615: F, t11555: F, t1053: F, t1102: F, t1103: F, t7028: F, t2850: F, t4176: F) -> (F, F, F, F) {
    let t39300 = t39299 * t795;
    let t39303 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3276 * t39300;
    let t39306 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t10615 * t11555;
    let t39309 = t1102 * t1053 * t1103 * t7028;
    let t39311 = t4176 * t2850;
    (t39303, t39306, t39309, t39311)
}
