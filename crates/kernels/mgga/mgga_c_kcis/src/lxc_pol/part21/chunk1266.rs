//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1266/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1266<F: Float>(t7749: F, t95416: F, t28059: F, t3339: F, t1196: F, t13181: F, t8069: F, t92540: F, t26946: F, t28045: F, t26933: F, t28050: F) -> (F, F, F, F, F, F) {
    let t95417 = t95416 * t7749;
    let t95419 = t28059 * t3339;
    let t95421 = t13181 * t1196;
    let t95423 = t92540 * t8069;
    let t95425 = t28045 * t26946;
    let t95427 = t26933 * t28050;
    (t95417, t95419, t95421, t95423, t95425, t95427)
}
