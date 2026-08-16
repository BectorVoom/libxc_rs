//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 962/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk962<F: Float>(t16359: F, t21625: F, t1319: F, t16582: F, t22114: F, t3255: F, t7222: F, t3780: F, t531: F, t1650: F, t1897: F, t11634: F, t1419: F) -> (F, F, F, F, F, F) {
    let t22184 = t16359 * t21625;
    let t22188 = t16582 * t22114 * t1319;
    let t22191 = t3255 * t7222;
    let t22193 = t3780 * t531;
    let t22194 = t1650 * t1897;
    let t22196 = t22193 * t22194 * t1319;
    let t22200 = t11634 * t22194 * t1419;
    (t22184, t22188, t22191, t22194, t22196, t22200)
}
