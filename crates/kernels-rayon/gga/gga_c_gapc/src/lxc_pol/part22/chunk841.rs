//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 841/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk841(t2300: f64, t2636: f64, t3396: f64, t2979: f64, t7624: f64, t2255: f64, t2982: f64, t2619: f64, t9128: f64, t3388: f64, t916: f64, t3392: f64) -> (f64, f64, f64, f64, f64) {
    let t9620 = t2636 * t2300;
    let t9621 = t3396 * t9620;
    let t9623 = t7624 * t2979;
    let t9624 = t2982 * t2255;
    let t9625 = t9623 * t9624;
    let t9627 = t2619 * t9128;
    let t9628 = t9627 * t3388;
    let t9630 = t916 * t9128;
    let t9631 = t9630 * t3392;
    (t9621, t9624, t9625, t9628, t9631)
}
