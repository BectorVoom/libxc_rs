//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1096/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1096(t2595: f64, t38892: f64, t12272: f64, t7324: f64, t41574: f64, t41575: f64, t42906: f64, t46846: f64, t46847: f64, t46848: f64, t47063: f64, t47080: f64, t47083: f64) -> f64 {
    let t47085 = t38892 * t2595;
    let t47087 = t7324 * t12272;
    let t47089 = -6.0_f64 * t47080 + t46846 + 2.0_f64 * t47083 + t42906 + t46847 - t46848 + 2.0_f64 * t47085 + 2.0_f64 * t47087 + t41574 + t41575 - t47063;
    t47089
}
