//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2229/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229<F: Float>(t40722: F, t40726: F, t12858: F, t2528: F, t2371: F, t40729: F, t40733: F, t2745: F, t776: F, t4205: F, t9909: F, t2553: F, t868: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46228 = F::cast_from(0.17090684152272775383e-2_f64) * t40722;
    let t46232 = F::cast_from(24.0_f64) * t40726;
    let t46234 = t12858 * t2528;
    let t46235 = F::cast_from(0.51947577317044391276e2_f64) * t46234;
    let t46236 = t12858 * t2371;
    let t46237 = F::cast_from(0.35089341735807877242e1_f64) * t46236;
    let t46238 = F::cast_from(36.0_f64) * t40729;
    let t46239 = F::cast_from(0.10526802520742363173e2_f64) * t40733;
    let t46240 = t2745 * t776;
    let t46244 = t4205 * t9909;
    let t46245 = F::cast_from(12.0_f64) * t46244;
    let t46252 = t2553 * t868;
    (t46228, t46232, t46235, t46237, t46238, t46239, t46240, t46245, t46252)
}
