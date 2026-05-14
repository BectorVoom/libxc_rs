//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 813/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk813<F: Float>(t46849: F, t6508: F, t1358: F, t6507: F, t2321: F, t38674: F, t9074: F, t1365: F, t38281: F, t38277: F, t4261: F, t13749: F, t203: F, t550: F, t158: F, t123: F, t488: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46850 = t6508 * t46849;
    let t46852 = t1358 * t6507 * t46850;
    let t46859 = t9074 * t38674 * t2321;
    let t46862 = t9074 * t1365 * t38281;
    let t46865 = t9074 * t4261 * t38277;
    let t46867 = t203 * t13749;
    let t46868 = t550 * t46867;
    let t46871 = 0.31616674039640166221e-2 * t1358 * t1365 * t46868;
    let t46873 = t158 * t13749;
    let t46877 = 0.31616674039640166221e-2 * t1358 * t46873 * t123 * t488;
    (t46850, t46852, t46859, t46862, t46865, t46867, t46868, t46871, t46873, t46877)
}
