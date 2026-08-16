//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 615/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk615(t12362: f64, t12571: f64, t157: f64, t526: f64, t1045: f64, t2101: f64, t2178: f64, t358: f64, t1055: f64, t8232: f64, t1030: f64, t167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13119 = 4.0_f64 / 27.0_f64 * t12362;
    let t13123 = 4.0_f64 / 9.0_f64 * t12571;
    let t13140 = t526 * t157;
    let t13153 = t2101 * t1045;
    let t13165 = t2178 * t358;
    let t13187 = t8232 * t1055;
    let t13201 = t8232 * t1030;
    let t13208 = t2101 * t167;
    (t13119, t13123, t13140, t13153, t13165, t13187, t13201, t13208)
}
