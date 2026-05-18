//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 639/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk639<F: Float>(t7155: F, t7189: F, t1404: F, t1924: F, t1979: F, t4018: F, t4023: F, t486: F, t510: F, t538: F, t5787: F, t5799: F, t7028: F, t7113: F, t7116: F, t7119: F, t7123: F, t7142: F) -> (F, F) {
    let t7190 = t7155 + t7189;
    let t7192 = t4018 + F::new(0.46853067927761790996e-2) * t5787 + F::new(0.93706135855523581992e-2) * t5799 + F::new(0.46853067927761790996e-2) * t4023 * t7113 + F::new(0.93706135855523581992e-2) * t1404 * t7116 - F::new(0.23426533963880895498e-2) * t1404 * t7119 + F::new(0.14055920378328537299e-1) * t510 * t7123 - F::new(0.46853067927761790996e-2) * t510 * t7142 - t7028 * t538 - F::new(2.0) * t1924 * t1979 - t486 * t7190;
    (t7190, t7192)
}
