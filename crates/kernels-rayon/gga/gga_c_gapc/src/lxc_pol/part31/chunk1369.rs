//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1369/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1369(t33263: f64, t33265: f64, t33270: f64, t33275: f64, t33278: f64, t33284: f64, t33289: f64, t33292: f64, t33295: f64, t33298: f64, t33301: f64, t33305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36526 = 0.21720231316129303386e-4_f64 * t33263;
    let t36527 = 0.43440462632258606772e-4_f64 * t33265;
    let t36528 = 0.88465285289519332099e-6_f64 * t33270;
    let t36529 = 0.10117831965157323855e-7_f64 * t33275;
    let t36530 = 0.25002399603899953676e-2_f64 * t33278;
    let t36534 = 0.75031332402051115813e-8_f64 * t33284;
    let t36535 = 0.63350674672043801542e-5_f64 * t33289;
    let t36536 = 0.2318836277704281739e-4_f64 * t33292;
    let t36537 = 0.43440462632258606772e-4_f64 * t33295;
    let t36538 = 0.43440462632258606772e-4_f64 * t33298;
    let t36539 = 0.21720231316129303386e-4_f64 * t33301;
    let t36540 = 0.17632363114482012216e-5_f64 * t33305;
    (t36526, t36527, t36528, t36529, t36530, t36534, t36535, t36536, t36537, t36538, t36539, t36540)
}
