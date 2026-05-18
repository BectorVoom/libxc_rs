//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 906/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk906<F: Float>(t46254: F, t6717: F, t6914: F, t2386: F, t3516: F, t544: F, t6514: F, t204: F, t2476: F, t46094: F, t46115: F, t587: F) -> (F, F, F, F) {
    let t46257 = F::new(0.62115540045351614476e2) * t6914 * t6717 * t46254;
    let t46261 = F::new(0.53625734927775640005e1) * t544 * t6514 * t3516 * t2386;
    let t46264 = F::new(0.92023022289409799224e1) * t2476 * t204 * t46094;
    let t46267 = F::new(0.18404604457881959845e2) * t587 * t204 * t46115;
    (t46257, t46261, t46264, t46267)
}
