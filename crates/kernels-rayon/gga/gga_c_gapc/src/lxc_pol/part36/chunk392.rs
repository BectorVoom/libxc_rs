//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 392/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk392(t1900: f64, t1903: f64, t1743: f64, t198: f64, t199: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1904 = t1900 * t1903;
    let t1905 = t1743 * t1904;
    let t1906 = pi * t198;
    let t1907 = t199 * t199;
    let t1908 = 1.0_f64 / t1907;
    (t1904, t1905, t1906, t1907, t1908)
}
