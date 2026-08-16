//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 838/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk838(t3421: f64, t8392: f64, t1045: f64, t2101: f64, t2224: f64, t3446: f64, t9419: f64, t3565: f64, t604: f64, t379: f64, t2210: f64, t2178: f64, t358: f64) -> (f64, f64, f64, f64, f64) {
    let t13152 = 2.0_f64 / 27.0_f64 * t8392 * t3421;
    let t13153 = t2101 * t1045;
    let t13154 = t13153 * t2224;
    let t13157 = t9419 * t3446;
    let t13160 = t604 * t3565;
    let t13161 = t13160 * t379;
    let t13162 = t2210 * t13161;
    let t13165 = t2178 * t358;
    (t13152, t13154, t13157, t13162, t13165)
}
