//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2259/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2259(t13017: f64, t2563: f64, t1489: f64, t41083: f64, t2559: f64, t4126: f64, t4130: f64, t12997: f64, t13000: f64, t2566: f64, t67: f64, t792: f64, t9558: f64) -> (f64, f64, f64, f64, f64) {
    let t46788 = t2563 * t13017;
    let t46790 = t41083 * t1489;
    let t46793 = t2559 * t4126 * t4130;
    let t46794 = 0.11666666666666666666e0_f64 * t46793;
    let t46796 = t2566 * t12997 * t13000;
    let t46799 = t792 * t9558 * t67;
    (t46788, t46790, t46794, t46796, t46799)
}
