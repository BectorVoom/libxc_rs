//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2227/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2227<F: Float>(t46171: F, t46190: F, t157: F, t182: F, t40687: F, t4199: F, t9905: F, t2517: F, t3966: F, t707: F, t9494: F, t13471: F, t870: F) -> (F, F, F, F, F, F, F) {
    let t46191 = t46171 + t46190;
    let t46194 = F::cast_from(0.19751673498613801407e-1_f64) * t46191 * t157 * t182;
    let t46195 = F::cast_from(12.0_f64) * t40687;
    let t46196 = t4199 * t9905;
    let t46197 = F::cast_from(0.35089341735807877242e1_f64) * t46196;
    let t46206 = t707 * t2517 * t3966;
    let t46207 = F::cast_from(12.0_f64) * t46206;
    let t46208 = t4199 * t9494;
    let t46209 = F::cast_from(0.10254018858216406658e4_f64) * t46208;
    let t46213 = t13471 * t870;
    (t46191, t46194, t46195, t46197, t46207, t46209, t46213)
}
