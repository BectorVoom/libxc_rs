//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1205/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1205<F: Float>(t3435: F, t3441: F, t1136: F, t15722: F, t4825: F, t11195: F, t604: F, t670: F, t4794: F, t4822: F, t11196: F, t1689: F, t5211: F, t5217: F, t11700: F, t1904: F) -> (F, F, F, F, F, F, F, F) {
    let t44167 = t3435 * t3441;
    let t44181 = t1136 * t15722;
    let t44406 = t4825 * t4825;
    let t44407 = 1.0 / t44406;
    let t46460 = t604 / t11195 / t670;
    let t46925 = t4794 * t4822;
    let t46928 = t1689 * t11196;
    let t47019 = t5211 * t5217;
    let t47024 = t1904 * t11700;
    (t44167, t44181, t44407, t46460, t46925, t46928, t47019, t47024)
}
