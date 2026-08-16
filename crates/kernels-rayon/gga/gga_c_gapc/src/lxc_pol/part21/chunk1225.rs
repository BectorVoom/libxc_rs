//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1225/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1225(t11388: f64, t3065: f64, t11479: f64, t1912: f64, t5285: f64, t11326: f64, t8885: f64, t35135: f64, t35137: f64, t35141: f64, t35143: f64, t35146: f64, t35149: f64, t35152: f64, t35155: f64) -> f64 {
    let t35157 = t11388 * t3065;
    let t35160 = t5285 * t11479 * t1912;
    let t35162 = t11326 * t8885;
    let t35164 = -0.21642471925239962898e-3_f64 * t35135 - 0.16217772716043213195e-2_f64 * t35137 - 0.30775559784820528656e-8_f64 * t35141 - 0.13506074236995523433e-5_f64 * t35143 + 0.5686343261418565457e-6_f64 * t35146 - 0.32228090843368550272e-8_f64 * t35149 + 0.168651611569216142e-8_f64 * t35152 + 0.27665946779727057415e-8_f64 * t35155 + 0.49522272202316919254e-5_f64 * t35157 + 0.16908181191593721013e-5_f64 * t35160 - 0.40096157891080460192e-6_f64 * t35162;
    t35164
}
