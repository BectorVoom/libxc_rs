//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 844/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk844<F: Float>(t42189: F, t10526: F, t20471: F, t46094: F, t2487: F, t46115: F, t6711: F, t2386: F, t3529: F, t544: F, t6514: F, t204: F, t46362: F, t587: F, t4391: F, t46254: F, t6964: F) -> (F, F, F, F, F, F, F) {
    let t46564 = 0.17875244975925213335e0 * t42189;
    let t46567 = 0.21450293971110256001e1 * t20471 * t10526 * t46094;
    let t46570 = 0.87421871174939309262e2 * t2487 * t6711 * t46115;
    let t46574 = 0.25025342966295298669e1 * t544 * t6514 * t3529 * t2386;
    let t46577 = 0.18404604457881959845e2 * t587 * t204 * t46362;
    let t46580 = 0.14953741122029092374e3 * t2487 * t6711 * t46362;
    let t46583 = 0.42900587942220512003e1 * t4391 * t6964 * t46254;
    (t46564, t46567, t46570, t46574, t46577, t46580, t46583)
}
