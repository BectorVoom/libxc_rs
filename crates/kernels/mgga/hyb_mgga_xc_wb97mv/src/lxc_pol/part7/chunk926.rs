//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 926/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk926<F: Float>(t2025: F, t3300: F, t683: F, t1312: F, t762: F, t674: F, t2013: F, t3299: F, t2039: F, t214: F, t3288: F, t3177: F, t2035: F, t2038: F, t3141: F, t6487: F, t6492: F, t6495: F, t6720: F, t686: F, t8440: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8561 = t683 * t2025 * t3300 / 96.0;
    let t8562 = t762 * t1312;
    let t8563 = t8562 * t674;
    let t8567 = t3299 * t2013;
    let t8571 = t3299 * t2039;
    let t8580 = t3288 * t214;
    let t8581 = t8580 * t674;
    let t8585 = t3177 * t2013;
    let t8589 = t3177 * t2039;
    let t8593 = -t6720 / 96.0 - t8561 - t683 * t686 * t8563 / 32.0 - t683 * t686 * t8567 / 64.0 - t2035 * t2038 * t8571 / 48.0 + 3.0 / 32.0 * t8440 * t3141 - t6487 / 32.0 + t6492 / 48.0 - t6495 / 64.0 - t683 * t686 * t8581 / 32.0 - t683 * t686 * t8585 / 64.0 - t2035 * t2038 * t8589 / 48.0;
    (t8561, t8562, t8563, t8567, t8571, t8580, t8581, t8585, t8589, t8593)
}
