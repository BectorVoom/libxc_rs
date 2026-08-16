//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 984/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk984(t848: f64, t871: f64, t2883: f64, t3699: f64, t2409: f64, t4150: f64, t2881: f64, t15129: f64, t296: f64, t319: f64, t668: f64, t835: f64) -> (f64, f64, f64, f64) {
    let t15254 = t848 * t871;
    let t15255 = t3699 * t2883;
    let t15256 = t15254 * t15255;
    let t15259 = t4150 * t2409;
    let t15260 = t2881 * t15259;
    let t15263 = t296 * t15129;
    let t15267 = t835 * t319 * t668;
    (t15256, t15260, t15263, t15267)
}
