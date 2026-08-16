//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 657/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk657(t1637: f64, t599: f64, t89: f64, t143: f64, t7954: f64, t1882: f64, t2144: f64, t2170: f64, t8805: f64, t9068: f64, t8799: f64, t8802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9321 = t89 * t1637 * t599;
    let t9327 = t7954 * t143;
    let t9340 = t1882 * t2144;
    let t9342 = t1882 * t2170;
    let t9366 = 2.0_f64 / 3.0_f64 * t8805;
    let t9370 = t9068 / 3.0_f64;
    let t9372 = t8799 / 9.0_f64;
    let t9373 = 2.0_f64 / 27.0_f64 * t8802;
    (t9321, t9327, t9340, t9342, t9366, t9370, t9372, t9373)
}
