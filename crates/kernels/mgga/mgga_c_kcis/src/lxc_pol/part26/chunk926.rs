//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 926/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk926<F: Float>(t1409: F, t21585: F, t5526: F, t5792: F, t17057: F, t1961: F, t7119: F, t833: F, t6284: F, t1419: F, t7123: F, t11939: F, t7122: F) -> (F, F, F, F, F, F, F) {
    let t21586 = t1409 * t21585;
    let t21594 = t5792 * t5526;
    let t21597 = t17057 * t1961;
    let t21600 = t7119 * t833;
    let t21603 = t1409 * t6284;
    let t21604 = t21603 * t1419;
    let t21607 = t7123 * t833;
    let t21610 = t11939 * t7122;
    (t21586, t21594, t21597, t21600, t21604, t21607, t21610)
}
