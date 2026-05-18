//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 832/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk832<F: Float>(t3393: F, t3416: F, t3402: F, t1157: F, t752: F, t3407: F, t3166: F, t330: F, t3412: F, t1160: F, t318: F, t86: F) -> (F, F, F, F, F, F, F) {
    let t10552 = t3393 * t3416;
    let t10554 = t3393 * t3402;
    let t10556 = t752 * t1157;
    let t10558 = t3393 * t3407;
    let t10594 = t3166 * t330;
    let t10599 = t3393 * t3412;
    let t10631 = t86 * t318 * t1160;
    (t10552, t10554, t10556, t10558, t10594, t10599, t10631)
}
