//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 892/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk892<F: Float>(t2248: F, t419: F, t934: F, t3139: F, t959: F, t2999: F, t89: F, t943: F, t3000: F, t921: F, t8417: F, t971: F) -> (F, F, F, F, F) {
    let t45662 = t419 * t2248 * t934;
    let t46019 = t3139 * t959;
    let t46256 = t89 * t2999 * t943;
    let t46320 = t89 * t3000 * t921;
    let t46565 = t971 * t8417;
    (t45662, t46019, t46256, t46320, t46565)
}
