//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1268/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1268<F: Float>(t7968: F, t99236: F, t99175: F, t4479: F, t8236: F, t1505: F, t28556: F, t1628: F, t28869: F, t28326: F, t28878: F, t28881: F) -> (F, F, F, F, F, F, F, F) {
    let t99676 = F::new(0.30918233506944444444e-4) * t7968 * t99236;
    let t99678 = t7968 * t99175;
    let t99718 = t8236 * t4479;
    let t99724 = t28556 * t1505;
    let t99730 = t28869 * t1628;
    let t99790 = t28326 / F::new(8.0);
    let t99791 = t28878 / F::new(8.0);
    let t99792 = t28881 / F::new(8.0);
    (t99676, t99678, t99718, t99724, t99730, t99790, t99791, t99792)
}
