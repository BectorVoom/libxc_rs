//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 418/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk418(t2200: f64, t831: f64, t1454: f64, t268: f64, t282: f64, t285: f64) -> (f64, f64, f64, f64) {
    let t2201 = t2200 * t831;
    let t2204 = t1454 * t268;
    let t2205 = t285 * t282;
    let t2206 = 1.0_f64 / t2205;
    (t2201, t2204, t2205, t2206)
}
