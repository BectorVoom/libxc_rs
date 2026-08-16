//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1009/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1009(t2801: f64, t39337: f64, t14289: f64, t501: f64, t605: f64, t3718: f64, t8042: f64, t1382: f64, t14348: f64, t1052: f64, t12270: f64, t14350: f64, t14408: f64, t1960: f64, t46004: f64, t46006: f64, t46008: f64, t46013: f64, t46016: f64, t46019: f64, t46023: f64, t46025: f64, t46832: f64, t46835: f64, t50781: f64, t5559: f64, t841: f64) -> (f64, f64, f64, f64, f64) {
    let t50789 = 4.0_f64 * t39337 * t2801;
    let t50790 = t14289 * t501;
    let t50791 = t50790 * t605;
    let t50796 = 2.0_f64 * t8042 * t3718;
    let t50799 = 2.0_f64 * t1382 * t14348 * t605;
    let t50800 = 4.0_f64 * t1052 * t12270 * t1960 - 12.0_f64 * t14350 * t5559 * t841 + 2.0_f64 * t14408 * t1960 * t841 + t46004 - t46006 + t46008 + t46013 - t46016 + t46019 - t46023 - t46025 + t46832 - t46835 + t50781 - t50789 + t50791 + t50796 - t50799;
    (t50789, t50791, t50796, t50799, t50800)
}
