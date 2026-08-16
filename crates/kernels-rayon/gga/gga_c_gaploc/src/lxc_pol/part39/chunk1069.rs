//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1069/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1069(t46845: f64, t605: f64, t1377: f64, t13836: f64, t41572: f64, t3689: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t46846 = t46845 * t605;
    let t46847 = t1377 * t13836;
    let t46848 = 2.0_f64 * t41572;
    let t46849 = t3689 * t874;
    (t46846, t46847, t46848, t46849)
}
