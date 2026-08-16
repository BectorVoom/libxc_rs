//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 964/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk964(t30268: f64, t8956: f64, t1983: f64, t30262: f64, t7586: f64, t8406: f64, t4680: f64, t7346: f64, t8896: f64, t7433: f64, t8962: f64, t30374: f64, t8657: f64) -> (f64, f64, f64, f64, f64) {
    let t34107 = t30268 * t8956;
    let t34127 = t30262 * t7586 * t1983 * t8406;
    let t34130 = t7346 * t4680 * t8896;
    let t34132 = t7433 * t8962;
    let t34156 = t30374 * t8657;
    (t34107, t34127, t34130, t34132, t34156)
}
