//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1071/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1071<F: Float>(t2321: F, t38674: F, t9074: F, t1365: F, t38281: F, t38277: F, t4261: F, t13749: F, t203: F, t550: F, t1358: F, t158: F) -> (F, F, F, F, F, F, F) {
    let t46859 = t9074 * t38674 * t2321;
    let t46862 = t9074 * t1365 * t38281;
    let t46865 = t9074 * t4261 * t38277;
    let t46867 = t203 * t13749;
    let t46868 = t550 * t46867;
    let t46871 = F::new(0.31616674039640166221e-2) * t1358 * t1365 * t46868;
    let t46873 = t158 * t13749;
    (t46859, t46862, t46865, t46867, t46868, t46871, t46873)
}
