//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 629/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk629(t1648: f64, t458: f64, t1652: f64, t17: f64, t7760: f64, t1594: f64, t7858: f64, t62: f64, t66: f64, t1613: f64, t77: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7950 = t458 * t1648;
    let t7952 = t458 * t1652;
    let t7954 = t17 * t7760;
    let t7982 = t1594 * t7858;
    let t7983 = t62 * t66;
    let t7998 = t77 * t1613;
    let t7999 = t7998 * t373;
    (t7950, t7952, t7954, t7982, t7983, t7999)
}
