//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 879/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk879(t30401: f64, t30409: f64, t30418: f64, t322: f64, t130: f64, t3558: f64, t145: f64, t154: f64, t19: f64, t3157: f64, t661: f64, t1165: f64, t3809: f64, t604: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t30421 = t30401 * t30418 * t30409 * t322;
    let t30422 = 0.1886885537376249124e-2_f64 * t30421;
    let t30423 = t130 * t3558;
    let t30428 = t30423 * t154 * t3157 * t145 * t19 * t661;
    let t30429 = 5.0_f64 / 576.0_f64 * t30428;
    let t30444 = t7493 * t1165 * t604 * t3809;
    (t30422, t30423, t30429, t30444)
}
