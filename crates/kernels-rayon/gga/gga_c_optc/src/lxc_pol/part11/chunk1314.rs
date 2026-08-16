//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1314/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1314(t39565: f64, t49404: f64, t49406: f64, t57057: f64, t57060: f64, t57063: f64, t57066: f64, t57069: f64, t57071: f64, t57073: f64, t57100: f64, t57102: f64, t57104: f64, t57106: f64) -> f64 {
    let t57447 = 0.13892666666666666667e1_f64 * t39565 + 0.166712e1_f64 * t49404 - 0.55570666666666666668e0_f64 * t49406 - 0.516475e0_f64 * t57057 + 0.123954e2_f64 * t57060 + 0.309885e1_f64 * t57063 + 0.94674375e0_f64 * t57066 + 0.2366859375e0_f64 * t57069 - 0.52945875e1_f64 * t57071 - 0.6618234375e1_f64 * t57073 + 0.6311625e0_f64 * t57100 + 0.158837625e2_f64 * t57102 - 0.705945e1_f64 * t57104 + 0.3529725e1_f64 * t57106;
    t57447
}
