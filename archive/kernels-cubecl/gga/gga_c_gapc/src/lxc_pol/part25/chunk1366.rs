//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1366/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1366<F: Float>(t33263: F, t33265: F, t33270: F, t33275: F, t33278: F, t33284: F, t33289: F, t33292: F, t33295: F, t33298: F, t33301: F, t33305: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36526 = F::cast_from(0.21720231316129303386e-4_f64) * t33263;
    let t36527 = F::cast_from(0.43440462632258606772e-4_f64) * t33265;
    let t36528 = F::cast_from(0.88465285289519332099e-6_f64) * t33270;
    let t36529 = F::cast_from(0.10117831965157323855e-7_f64) * t33275;
    let t36530 = F::cast_from(0.25002399603899953676e-2_f64) * t33278;
    let t36534 = F::cast_from(0.75031332402051115813e-8_f64) * t33284;
    let t36535 = F::cast_from(0.63350674672043801542e-5_f64) * t33289;
    let t36536 = F::cast_from(0.2318836277704281739e-4_f64) * t33292;
    let t36537 = F::cast_from(0.43440462632258606772e-4_f64) * t33295;
    let t36538 = F::cast_from(0.43440462632258606772e-4_f64) * t33298;
    let t36539 = F::cast_from(0.21720231316129303386e-4_f64) * t33301;
    let t36540 = F::cast_from(0.17632363114482012216e-5_f64) * t33305;
    (t36526, t36527, t36528, t36529, t36530, t36534, t36535, t36536, t36537, t36538, t36539, t36540)
}
