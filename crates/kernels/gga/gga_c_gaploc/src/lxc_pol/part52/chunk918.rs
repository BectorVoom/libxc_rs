//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 918/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk918<F: Float>(t42071: F, t46121: F, t544: F, t40166: F, t10525: F, t2365: F, t35959: F, t13420: F, t4614: F, t574: F, t37326: F, t895: F) -> (F, F, F, F, F, F) {
    let t46498 = F::new(0.51123901271894332902e0) * t42071;
    let t46499 = t544 * t46121;
    let t46500 = t46499 * t40166;
    let t46501 = F::new(0.17875244975925213335e0) * t46500;
    let t46503 = t10525 * t2365 * t35959;
    let t46504 = F::new(0.89376224879626066674e-1) * t46503;
    let t46507 = F::new(0.12269736305254639897e2) * t574 * t4614 * t13420;
    let t46516 = F::new(0.23833659967900284446e0) * t895 * t37326;
    (t46498, t46499, t46501, t46504, t46507, t46516)
}
