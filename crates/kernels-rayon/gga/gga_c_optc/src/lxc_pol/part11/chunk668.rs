//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 668/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk668(t50: f64, t5262: f64, t5478: f64, t4573: f64, t5239: f64, t38: f64, t620: f64, t22: f64, t34: f64, t39: f64, t88: f64, t35: f64, t543: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t5479 = t5262 + t5478;
    let t5483 = piecewise3(t51, 0.0_f64, t4573);
    let t6116 = t5239 * rho1;
    let t6163 = t38 * t620;
    let t6165 = 1.0_f64 / t22 / t6163;
    let t6316 = t34 * t39;
    let t6318 = 24.0_f64 * t6316 * t88;
    let t6319 = t35 * t543;
    (t5479, t5483, t6116, t6163, t6165, t6316, t6318, t6319)
}
