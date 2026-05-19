//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 933/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk933<F: Float>(t122: F, t160: F, t2379: F, t2434: F, t2439: F, t2440: F, t2459: F, t2477: F, t60: F, t684: F, t718: F, t728: F, t745: F, t8556: F, t8561: F, t8604: F, t8747: F, t8969: F, t8972: F, t8979: F, t8996: F, t97: F) -> F {
    let t8998 = -F::cast_from(0.70279601891642686494e-2_f64) * t160 * t97 - F::cast_from(0.14055920378328537299e-1_f64) * t8969 * t728 - F::cast_from(0.21083880567492805948e-1_f64) * t8972 * t2440 + F::cast_from(0.70279601891642686494e-2_f64) * t2434 * t2459 - F::cast_from(0.28111840756657074598e-1_f64) * t8979 * t8556 + F::cast_from(0.21083880567492805948e-1_f64) * t2439 * t8561 - F::cast_from(0.23426533963880895498e-2_f64) * t718 * t8604 - t8747 * t122 - F::new(3.0) * t2379 * t745 - F::new(3.0) * t684 * t2477 - t60 * t8996;
    t8998
}
