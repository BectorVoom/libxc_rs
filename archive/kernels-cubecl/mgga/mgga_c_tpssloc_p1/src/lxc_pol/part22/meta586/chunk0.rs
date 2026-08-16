//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2097/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2097<F: Float>(t4199: F, t9919: F, t9892: F, t13123: F, t9882: F, t9888: F, t118: F, t2375: F, t4095: F, t9905: F, t2517: F, t3966: F, t707: F) -> (F, F, F, F, F, F, F) {
    let t46125 = t4199 * t9919;
    let t46130 = t4199 * t9892;
    let t46132 = t13123 * t9882;
    let t46134 = t13123 * t9888;
    let t46137 = t4095 * t118 * t2375;
    let t46138 = F::cast_from(0.32530743900905219526e-1_f64) * t46137;
    let t46196 = t4199 * t9905;
    let t46206 = t707 * t2517 * t3966;
    (t46125, t46130, t46132, t46134, t46138, t46196, t46206)
}
