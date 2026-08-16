//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 789/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk789(t3902: f64, t5120: f64, t91: f64, t1154: f64, t5092: f64, t9890: f64, t192: f64, t21399: f64, t743: f64, t20489: f64, t738: f64, t737: f64) -> (f64, f64, f64, f64, f64) {
    let t21556 = t91 * t3902 * t5120;
    let t21565 = t5092 * t1154;
    let t21567 = t91 * t9890 * t21565;
    let t21570 = t192 * t743 * t21399;
    let t21572 = t738 * t20489;
    let t21573 = t737 * t21572;
    (t21556, t21567, t21570, t21572, t21573)
}
