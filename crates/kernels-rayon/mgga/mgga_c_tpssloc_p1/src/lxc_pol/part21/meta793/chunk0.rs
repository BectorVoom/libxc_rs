//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2754/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2754(t40738: f64, t40745: f64, t46283: f64, t46285: f64, t13133: f64, t4202: f64, t5597: f64, t9912: f64, t40754: f64, t40761: f64, t46291: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58025 = 0.43374325201206959367e-1_f64 * t40738;
    let t58026 = 0.10843581300301739842e-1_f64 * t40745;
    let t58027 = 48.0_f64 * t46283;
    let t58028 = 8.0_f64 * t46285;
    let t58030 = 16.0_f64 * t13133 * t4202;
    let t58032 = 4.0_f64 * t9912 * t5597;
    let t58033 = 0.20779030926817756511e3_f64 * t40754;
    let t58034 = 0.20508037716432813316e4_f64 * t40761;
    let t58035 = 48.0_f64 * t46291;
    let t58036 = -t58025 - t40741 - t40743 + t58026 + t58027 + t58028 + t40748 + t58030 + t58032 + t58033 + t40760 - t58034 + t58035;
    (t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034, t58035, t58036)
}
