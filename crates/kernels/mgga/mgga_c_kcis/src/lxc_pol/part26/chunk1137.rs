//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1137/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1137<F: Float>(t98918: F, t27601: F, t28714: F, t7968: F, t99236: F, t99175: F, t4479: F, t8236: F, t1505: F, t28556: F, t1628: F, t28869: F, t28326: F, t28878: F, t28881: F, t28884: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t99667 = 0.15476481481481481481e-2 * t98918;
    let t99671 = 0.23168402777777777778e-3 * t28714 * t27601;
    let t99676 = 0.30918233506944444444e-4 * t7968 * t99236;
    let t99678 = t7968 * t99175;
    let t99718 = t8236 * t4479;
    let t99724 = t28556 * t1505;
    let t99730 = t28869 * t1628;
    let t99790 = t28326 / 8.0;
    let t99791 = t28878 / 8.0;
    let t99792 = t28881 / 8.0;
    let t99793 = t28884 / 8.0;
    (t99667, t99671, t99676, t99678, t99718, t99724, t99730, t99790, t99791, t99792, t99793)
}
