//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 950/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk950(t17449: f64, t8697: f64, t8700: f64, t1102: f64, t4224: f64, t5219: f64, t4305: f64, t5264: f64, t11604: f64, t11761: f64, t1220: f64, t14864: f64, t17363: f64, t17425: f64, t17429: f64, t17431: f64, t17433: f64, t17435: f64, t17438: f64, t17440: f64, t17443: f64, t17447: f64, t4230: f64, t5233: f64, t9229: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17450 = t8697 * t17449;
    let t17451 = t17450 * t8700;
    let t17453 = 0.1025389702100779493e4_f64 * t1102 * t17451;
    let t17454 = t4224 * t5219;
    let t17456 = 0.35089340384731224426e1_f64 * t1102 * t17454;
    let t17460 = 0.35089340384731224426e1_f64 * t4305 * t5264;
    let t17463 = t17363 + t17425 + t17429 + t17431 + t17433 + t17435 - t17438 + t1220 * t17440 + 14.0_f64 / 27.0_f64 * t1220 * t17443 + t9229 - t17447 - t14864 / 3.0_f64 - t17453 + t17456 - 100.0_f64 / 81.0_f64 * t11604 + 8.0_f64 / 9.0_f64 * t11761 + t17460 + 8.0_f64 / 3.0_f64 * t4230 * t5233;
    (t17451, t17453, t17454, t17456, t17460, t17463)
}
