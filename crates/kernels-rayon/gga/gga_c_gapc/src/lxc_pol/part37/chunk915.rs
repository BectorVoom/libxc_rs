//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 915/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk915(t10382: f64, t3189: f64, t132: f64, t3186: f64, t190: f64, t329: f64, t2536: f64, t10343: f64, t2405: f64, t493: f64, t3230: f64, t6808: f64, t996: f64) -> (f64, f64, f64, f64, f64) {
    let t10383 = t10382 * t3189;
    let t10385 = t132 * t3186;
    let t10386 = t10385 * t3189;
    let t10388 = t190 * t329;
    let t10389 = t10388 * t2536;
    let t10390 = t10343 * t10389;
    let t10392 = t493 * t2405;
    let t10393 = t3230 * t10392;
    let t10395 = t996 * t6808;
    (t10383, t10386, t10390, t10393, t10395)
}
