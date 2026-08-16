//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2104/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2104(t41189: f64, t4134: f64, t13025: f64, t9546: f64, t1489: f64, t41083: f64, t2559: f64, t4126: f64, t4130: f64, t12997: f64, t13000: f64, t2566: f64) -> (f64, f64, f64, f64, f64) {
    let t46772 = t41189 * t4134;
    let t46782 = t9546 * t13025;
    let t46783 = 0.15833333333333333333e-1_f64 * t46782;
    let t46790 = t41083 * t1489;
    let t46793 = t2559 * t4126 * t4130;
    let t46794 = 0.11666666666666666666e0_f64 * t46793;
    let t46796 = t2566 * t12997 * t13000;
    (t46772, t46783, t46790, t46794, t46796)
}
