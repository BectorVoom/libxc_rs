//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 387/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk387(t1702: f64, t554: f64, t1701: f64, t137: f64, t548: f64, t135: f64) -> (f64, f64, f64, f64, f64) {
    let t2044 = t1702 * t554;
    let t2045 = t1701 * t2044;
    let t2057 = 1.0_f64 / t548 / t137;
    let t2058 = t135 * t2057;
    let t2059 = t554 * t554;
    (t2044, t2045, t2057, t2058, t2059)
}
