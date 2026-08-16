//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 295/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk295(t3874: f64, t4002: f64, t258: f64, t3951: f64, t1217: f64, t2648: f64, t1186: f64, t2336: f64, t89: f64, t2857: f64, t3691: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t4003 = t3874 + t4002;
    let t4005 = t3951 * t258;
    let t4027 = t2648 * t1217;
    let t4032 = t89 * t2336 * t1186;
    let t4034 = t2857 * t3691;
    let t4035 = t446 * t4034;
    (t4003, t4005, t4027, t4032, t4035)
}
