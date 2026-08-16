//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 456/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk456(t2503: f64, t2504: f64, t604: f64, t820: f64, t1764: f64, t919: f64, t2387: f64, t282: f64, t129: f64, t825: f64, t869: f64) -> (f64, f64, f64, f64, f64) {
    let t2505 = t2503 * t2504;
    let t2508 = t604 * t820;
    let t2511 = t1764 * t919;
    let t2514 = t2387 * t282;
    let t2515 = t2514 * t129;
    let t2520 = t869 * t825;
    (t2505, t2508, t2511, t2515, t2520)
}
