//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 798/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk798<F: Float>(t46094: F, t6716: F, t6717: F, t42189: F, t10526: F, t20471: F, t2487: F, t46115: F, t6711: F, t2386: F, t3529: F, t544: F, t6514: F, t204: F, t46362: F, t587: F) -> (F, F, F, F, F, F) {
    let t46559 = 0.62115540045351614476e2 * t6716 * t6717 * t46094;
    let t46564 = 0.17875244975925213335e0 * t42189;
    let t46567 = 0.21450293971110256001e1 * t20471 * t10526 * t46094;
    let t46570 = 0.87421871174939309262e2 * t2487 * t6711 * t46115;
    let t46574 = 0.25025342966295298669e1 * t544 * t6514 * t3529 * t2386;
    let t46577 = 0.18404604457881959845e2 * t587 * t204 * t46362;
    (t46559, t46564, t46567, t46570, t46574, t46577)
}
