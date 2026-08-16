//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 567/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk567(t26: f64, t7943: f64, t1771: f64, t380: f64, t1644: f64, t458: f64, t1648: f64, t1652: f64, t17: f64, t7760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7944 = t26 * t7943;
    let t7945 = 28.0_f64 / 27.0_f64 * t7944;
    let t7946 = t1771 * t380;
    let t7948 = t458 * t1644;
    let t7950 = t458 * t1648;
    let t7952 = t458 * t1652;
    let t7954 = t17 * t7760;
    (t7944, t7945, t7946, t7948, t7950, t7952, t7954)
}
