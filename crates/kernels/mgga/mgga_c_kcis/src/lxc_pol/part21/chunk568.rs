//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 568/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk568<F: Float>(t1109: F, t4625: F, t345: F, t1098: F, t1762: F, t1727: F, t330: F, t829: F, t3274: F, t313: F, t934: F, t3293: F, t304: F, t346: F, t1045: F, t1728: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4626 = t1109 * t4625;
    let t4627 = t345 * t4626;
    let t4630 = t1098 * t1762;
    let t4632 = t1727 * t330;
    let t4633 = t4632 * t829;
    let t4634 = t3274 * t4633;
    let t4637 = t313 * t1727;
    let t4638 = t4637 * t934;
    let t4639 = t3293 * t4638;
    let t4642 = t304 * t346;
    let t4643 = t1728 * t1045;
    (t4626, t4627, t4630, t4632, t4634, t4637, t4638, t4639, t4642, t4643)
}
