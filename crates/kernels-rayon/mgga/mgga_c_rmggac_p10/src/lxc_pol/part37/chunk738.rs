//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 738/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk738(t69265: f64, t69267: f64, t69270: f64, t69272: f64, t69289: f64, t69067: f64, t69102: f64, t1986: f64, t2206: f64, t2209: f64, t2213: f64, t118: f64, t495: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71269 = 0.16852636469289804646e0_f64 * t69265;
    let t71270 = 0.56054872888033565654e-2_f64 * t69267;
    let t71271 = 0.11515968254014699841e0_f64 * t69270;
    let t71272 = 0.34974422104785384706e-1_f64 * t69272;
    let t71278 = 0.81431813490554440378e-3_f64 * t69289;
    let t71300 = 0.32526727992809621482e-4_f64 * t69067;
    let t71319 = 0.47433845426916398123e-7_f64 * t69102;
    let t71340 = t1986 * t2206;
    let t71343 = t1986 * t2209;
    let t71346 = t1986 * t2213;
    let t71366 = t1986 * t118 * t699 * t495;
    (t71269, t71270, t71271, t71272, t71278, t71300, t71319, t71340, t71343, t71346, t71366)
}
