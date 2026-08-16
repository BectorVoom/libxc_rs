//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1311/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311(t16693: f64, t20749: f64, t46376: f64, t16689: f64, t5597: f64, t39585: f64, t39590: f64, t39593: f64, t41254: f64, t75943: f64, t75950: f64, t75951: f64, t75952: f64) -> (f64, f64, f64, f64) {
    let t76017 = 144.0_f64 * t16693 * t20749;
    let t76018 = 0.23392894490538584828e1_f64 * t46376;
    let t76020 = 24.0_f64 * t16689 * t5597;
    let t76021 = t75943 - t39585 + t39590 - t39593 + t75950 + t75951 - t75952 + t76017 + t41254 - t76018 + t76020;
    (t76017, t76018, t76020, t76021)
}
