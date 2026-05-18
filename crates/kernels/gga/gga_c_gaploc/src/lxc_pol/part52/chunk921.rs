//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 921/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk921<F: Float>(t2299: F, t3529: F, t1415: F, t1646: F, t46094: F, t6716: F, t6717: F, t42189: F, t10526: F, t20471: F, t2487: F, t46115: F, t6711: F) -> (F, F, F, F, F) {
    let t46550 = t2299 * t3529;
    let t46553 = F::new(0.35750489951850426669e0) * t1415 * t46550 * t1646;
    let t46559 = F::new(0.62115540045351614476e2) * t6716 * t6717 * t46094;
    let t46564 = F::new(0.17875244975925213335e0) * t42189;
    let t46567 = F::new(0.21450293971110256001e1) * t20471 * t10526 * t46094;
    let t46570 = F::new(0.87421871174939309262e2) * t2487 * t6711 * t46115;
    (t46553, t46559, t46564, t46567, t46570)
}
