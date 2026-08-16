//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1578/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1578(t2955: f64, t969: f64, t2967: f64, t964: f64, t340: f64, t63: f64, t344: f64, t221: f64, t339: f64, t2960: f64, t2974: f64, t135: f64, t3016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10331 = t2955 * t969;
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3_f64 * t339 * t10337;
    let t10342 = t2960 * t2974;
    let t10352 = t135 * t3016;
    (t10331, t10333, t10335, t10337, t10339, t10342, t10352)
}
