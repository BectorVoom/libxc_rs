//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1099/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1099<F: Float>(t5633: F, t7931: F, t303: F, t553: F, t5757: F, t1459: F, t2012: F, t1014: F, t8179: F, t167: F, t7909: F, t16892: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28406 = t7931 * t5633;
    let t28407 = t303 * t28406;
    let t28409 = t553 * t5757;
    let t28410 = t303 * t28409;
    let t28412 = t1459 * t2012;
    let t28413 = t303 * t28412;
    let t28415 = t1014 * t8179;
    let t28419 = t7909 * t167;
    let t28420 = t16892 * t28419;
    (t28406, t28407, t28409, t28410, t28412, t28413, t28415, t28419, t28420)
}
