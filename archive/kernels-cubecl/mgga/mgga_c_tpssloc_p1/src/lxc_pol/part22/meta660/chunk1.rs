//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2204/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204<F: Float>(t212: F, t5544: F, t12998: F, t686: F, t776: F, t13012: F, t16798: F, t16773: F, t46843: F, t16777: F, t5527: F, t46799: F) -> (F, F, F, F, F, F, F) {
    let t59135 = t212 * t5544;
    let t59138 = t12998 * t686 * t59135 * t776;
    let t59140 = t13012 * t16798;
    let t59154 = t46843 * t16773;
    let t59156 = t13012 * t16777;
    let t59162 = t212 * t5527;
    let t59165 = t46799 * t686 * t59162 * t776;
    (t59135, t59138, t59140, t59154, t59156, t59162, t59165)
}
