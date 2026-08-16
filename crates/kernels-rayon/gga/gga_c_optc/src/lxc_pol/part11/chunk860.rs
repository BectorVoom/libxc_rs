//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 860/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk860(t16287: f64, t185: f64, t108: f64, t176: f64, t1303: f64, t13056: f64, t13573: f64, t13578: f64, t16341: f64, t16342: f64, t16344: f64, t16345: f64, t203: f64, t3308: f64, t6480: f64, t6484: f64, t6816: f64) -> (f64, f64) {
    let t16604 = t185 * t16287;
    let t16606 = t176 * t16604 * t108;
    let t16614 = t16341 + t16342 + t16344 + t6816 - t16345 + t16606 * t203 / 2.0_f64 - 0.77534644304710291488e-2_f64 * t3308 * t13056 * t1303 + 3.0_f64 * t13573 + 3.0_f64 / 2.0_f64 * t13578 - t6480 - t6484;
    (t16606, t16614)
}
