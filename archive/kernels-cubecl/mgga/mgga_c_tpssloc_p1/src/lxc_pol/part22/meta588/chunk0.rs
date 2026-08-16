//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2100/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2100<F: Float>(t13123: F, t9467: F, t4199: F, t9713: F, t1471: F, t31: F, t4211: F, t9874: F, t13119: F, t2663: F, t2517: F, t4098: F) -> (F, F, F, F, F, F) {
    let t46371 = t13123 * t9467;
    let t46376 = t4199 * t9713;
    let t46387 = t31 * t1471;
    let t46433 = t4211 * t9874;
    let t46435 = t13119 * t2663;
    let t46436 = F::cast_from(0.73245789224026180216e-3_f64) * t46435;
    let t46437 = t4098 * t2517;
    (t46371, t46376, t46387, t46433, t46436, t46437)
}
