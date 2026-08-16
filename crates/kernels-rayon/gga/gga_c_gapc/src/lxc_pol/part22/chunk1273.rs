//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1273/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1273(t11303: f64, t21842: f64, t11500: f64, t1717: f64, t144: f64, t21072: f64, t21076: f64, t26416: f64, t5542: f64, t3144: f64, t34465: f64, t11473: f64, t3060: f64, t3076: f64) -> (f64, f64, f64, f64, f64) {
    let t35203 = t11303 * t21842;
    let t35205 = t11500 * t1717;
    let t35210 = t21072 * t5542 * t26416 * t144 * t21076;
    let t35212 = t34465 * t3144;
    let t35215 = t3060 * t11473 * t3076;
    (t35203, t35205, t35210, t35212, t35215)
}
