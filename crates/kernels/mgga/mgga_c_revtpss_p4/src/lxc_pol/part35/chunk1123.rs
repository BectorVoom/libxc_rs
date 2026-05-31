//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1123/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1123<F: Float>(t45972: F, t7342: F, t10309: F, t26178: F, t94973: F, t25373: F, t26550: F, t25386: F, t26518: F, t9285: F, t25299: F, t2061: F, t22: F, t25402: F) -> (F, F, F, F, F, F, F, F) {
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95397 = F::cast_from(308.0_f64) / F::cast_from(27.0_f64) * t94973;
    let t95536 = t25373 * t26550;
    let t95537 = t25386 * t95536;
    let t95540 = t26518 * t9285;
    let t95542 = F::cast_from(0.68540937416128198417e-2_f64) * t25299 * t95540;
    let t95546 = t25402 * t2061 * t22;
    (t95316, t95319, t95397, t95536, t95537, t95540, t95542, t95546)
}
