//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 754/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk754<F: Float>(t11544: F, t11576: F, t11609: F, t11636: F, t10777: F, t11491: F, t11493: F, t11495: F, t11496: F, t11500: F, t11503: F, t11507: F, t11510: F, t1689: F, t1809: F, t1860: F, t4794: F, t5089: F, t5172: F, t604: F, t702: F) -> (F, F) {
    let t11638 = t11544 + t11576 + t11609 + t11636;
    let t11645 = -F::cast_from(0.28111840756657074597e-1_f64) * t11491 - F::cast_from(0.42167761134985611897e-1_f64) * t11493 - F::cast_from(0.14055920378328537299e-1_f64) * t11495 * t11496 - F::cast_from(0.28111840756657074597e-1_f64) * t5089 * t11500 + F::cast_from(0.14055920378328537299e-1_f64) * t5089 * t11503 + F::cast_from(0.14055920378328537299e-1_f64) * t1809 * t11507 + F::cast_from(0.14055920378328537299e-1_f64) * t1809 * t11510 - t604 * t11638 - F::new(3.0) * t1689 * t5172 - F::new(3.0) * t4794 * t1860 - t10777 * t702;
    (t11638, t11645)
}
