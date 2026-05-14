//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1214/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1214<F: Float>(t34484: F, t9725: F, t2804: F, t2807: F, t33212: F, t34055: F, t34058: F, t34061: F, t34462: F, t34466: F, t34469: F, t34474: F, t34477: F, t34480: F, t2028: F, t2063: F) -> (F, F, F) {
    let t34485 = t9725 * t34484;
    let t34487 = 0.13888888888888888889e-1 * t34462 * t2807 + 0.17361111111111111111e-2 * t34466 + 0.52083333333333333333e-2 * t2804 * t34469 + 0.17411041666666666666e-2 * t34055 - 0.52083333333333333333e-2 * t34474 * t2807 - 0.52083333333333333333e-2 * t34477 * t2807 - 0.17361111111111111111e-2 * t34480 - 0.17411041666666666666e-2 * t34058 - 0.17411041666666666666e-2 * t34061 + 0.6701388888888888889e-3 * t34485 - t33212;
    let t34494 = t2063 * t2028;
    (t34485, t34487, t34494)
}
