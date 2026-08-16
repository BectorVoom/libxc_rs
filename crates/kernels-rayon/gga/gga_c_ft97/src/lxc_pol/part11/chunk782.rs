//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 782/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk782(t1775: f64, t2778: f64, t2767: f64, t303: f64, t3051: f64, t1771: f64, t854: f64, t848: f64, t9909: f64, t4206: f64, t9592: f64, t10491: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10589 = t1775 * t2778;
    let t10591 = t1775 * t2767;
    let t10594 = 28.0_f64 / 27.0_f64 * t3051 * t303;
    let t10595 = t1771 * t854;
    let t10597 = t848 * t9909;
    let t10600 = t4206 * t9592;
    let t10603 = t10491 * t2;
    (t10589, t10591, t10594, t10595, t10597, t10600, t10603)
}
