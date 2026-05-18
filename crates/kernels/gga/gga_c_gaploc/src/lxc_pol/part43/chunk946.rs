//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 946/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk946<F: Float>(t46867: F, t550: F, t1358: F, t1365: F, t13749: F, t158: F, t123: F, t488: F, t13740: F, t2312: F, t2325: F, t38413: F, t882: F, t883: F) -> (F, F, F, F, F, F) {
    let t46868 = t550 * t46867;
    let t46871 = F::new(0.31616674039640166221e-2) * t1358 * t1365 * t46868;
    let t46873 = t158 * t13749;
    let t46877 = F::new(0.31616674039640166221e-2) * t1358 * t46873 * t123 * t488;
    let t46878 = t2312 * t13740;
    let t46884 = t882 * t2325 * t883 * t38413;
    (t46868, t46871, t46873, t46877, t46878, t46884)
}
