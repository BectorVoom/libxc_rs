//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 213/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk213(t60: f64, t20: f64, t66: f64, t63: f64, t72: f64, t684: f64, t209: f64, t691: f64, t75: f64, t78: f64, t124: f64, t138: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70 = 0.0_f64 < t60;
    let t695 = t66 * t20;
    let t696 = t63 * t695;
    let t697 = t72 * t72;
    let t698 = 1.0_f64 / t697;
    let t700 = piecewise3(t70, t684, -t684);
    let t702 = t209 * t698 * t700;
    let t705 = -7.0_f64 / 288.0_f64 * t63 * t691 * t75 - t696 * t702 / 96.0_f64;
    let t706 = 1.0_f64 / t78;
    let t707 = t705 * t706;
    let t710 = t66 * t124;
    let t712 = t86 * t710 * t138;
    (t696, t697, t698, t700, t702, t705, t706, t707, t712)
}
