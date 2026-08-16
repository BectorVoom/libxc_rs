//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 678/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk678(t2101: f64, t597: f64, t2224: f64, t160: f64, t2075: f64, t379: f64, t2221: f64, t2133: f64, t604: f64, t609: f64, t144: f64, t24: f64, t7368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9419 = t2101 * t597;
    let t9420 = t9419 * t2224;
    let t9424 = t160 * t2075 * t379;
    let t9425 = t2221 * t9424;
    let t9428 = t2133 * t604;
    let t9429 = t9428 * t609;
    let t9430 = t144 * t9429;
    let t9432 = t24 * t7368;
    (t9419, t9420, t9424, t9425, t9428, t9429, t9430, t9432)
}
