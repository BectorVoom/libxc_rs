//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 651/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk651(t23: f64, t7368: f64, t1642: f64, t525: f64, t1882: f64, t1971: f64, t1546: f64, t1975: f64, t89: f64, t1636: f64, t559: f64, t2076: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9016 = t23 * t7368;
    let t9049 = t1642 * t525;
    let t9059 = t1882 * t1971;
    let t9062 = t89 * t1546 * t1975;
    let t9065 = t89 * t1636 * t559;
    let t9068 = t89 * t375 * t2076;
    (t9016, t9049, t9059, t9062, t9065, t9068)
}
