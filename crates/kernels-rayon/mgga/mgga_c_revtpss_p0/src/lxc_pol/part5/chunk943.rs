//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 943/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk943(t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64, t701: f64, t682: f64) -> f64 {
    let t9305 = -0.25319e1_f64 * t9283 + 0.16879333333333333333e1_f64 * t9286 - 0.19692555555555555555e1_f64 * t9289 - 0.93011851851851851854e0_f64 * t9292 + 0.13651666666666666667e0_f64 * t9296 - 0.27303333333333333333e0_f64 * t9298 - 0.3185388888888888889e0_f64 * t9300 - 0.36514074074074074075e0_f64 * t9303;
    let t9306 = t9305 * t701;
    let t9308 = 1.0_f64 * t682 * t9306;
    t9308
}
