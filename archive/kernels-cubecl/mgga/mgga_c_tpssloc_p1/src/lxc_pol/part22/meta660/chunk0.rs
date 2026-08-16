//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2203/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2203<F: Float>(t16558: F, t707: F, t751: F, t16586: F, t9929: F, t16579: F, t172: F, t763: F, t67: F, t758: F, t16957: F, t41011: F) -> (F, F, F, F, F) {
    let t59037 = t707 * t751 * t16558;
    let t59039 = t9929 * t16586;
    let t59045 = t16579 * t172 * t763;
    let t59048 = t16579 * t67 * t758;
    let t59100 = t41011 * t16957;
    (t59037, t59039, t59045, t59048, t59100)
}
