//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 655/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk655(t1111: f64, t1133: f64, t1509: f64, t3081: f64, t3103: f64, t3116: f64, t3132: f64, t3140: f64, t4308: f64, t4315: f64, t4363: f64, t4366: f64, t5319: f64, t5325: f64, t5330: f64, t5333: f64, t5337: f64) -> f64 {
    let t5343 = -t1111 * t5319 / 144.0_f64 - t3081 - t3140 - 0.19318136643975017455e-1_f64 * t4366 - t4308 / 54.0_f64 + 0.47333755318775392234e-1_f64 * t3116 * t5325 + 0.9157278480459830169e1_f64 * t3103 * t5330 - 0.45786392402299150845e1_f64 * t3132 * t5333 - 0.36221506207453157728e-2_f64 * t1133 * t5337 - 0.37867004255020313788e0_f64 * t4363 * t1509 + t4315 / 432.0_f64;
    t5343
}
