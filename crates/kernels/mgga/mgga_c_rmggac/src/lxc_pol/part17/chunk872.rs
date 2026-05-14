//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 872/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk872<F: Float>(t1652: F, t2347: F, t262: F, t7788: F, t45731: F, t7785: F, t558: F, t8957: F, t1734: F, t2064: F, t793: F, t326: F, t35862: F, t35877: F, t40976: F, t41021: F, t41029: F, t41033: F, t41037: F, t41042: F, t41057: F) -> (F, F, F, F, F) {
    let t46357 = t2347 * t1652;
    let t46358 = t262 * t46357;
    let t46359 = t7788 * t46358;
    let t46361 = t7785 * t45731;
    let t46365 = t8957 * t558;
    let t46369 = t2064 * t1734;
    let t46370 = t793 * t46369;
    let t46372 = 0.72732431077987577944e-1 * t40976 + 0.66671395154821946449e-1 * t41021 - 0.20001418546446583934e0 * t41029 + 0.26668558061928778579e0 * t41033 + 0.20455996240684006296e-1 * t46359 + 0.81823984962736025184e-1 * t46361 - 0.72732431077987577943e-1 * t41037 - t41042 + 0.54549323308490683457e-1 * t41057 - 0.11974241701863808564e0 * t326 * t46365 - t35862 - 0.10000709273223291967e0 * t35877 - 0.79828278012425390427e-1 * t46370;
    (t46357, t46358, t46365, t46369, t46372)
}
