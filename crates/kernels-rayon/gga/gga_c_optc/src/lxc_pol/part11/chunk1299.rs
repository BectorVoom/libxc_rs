//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1299/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1299(t39565: f64, t49404: f64, t49406: f64, t57057: f64, t57060: f64, t57063: f64, t57066: f64, t57069: f64, t57071: f64, t57073: f64, t57100: f64, t57102: f64, t57104: f64, t57106: f64) -> f64 {
    let t57179 = 0.11038e1_f64 * t39565 + 0.132456e1_f64 * t49404 - 0.44152e0_f64 * t49406 - 0.301925e0_f64 * t57057 + 0.72462e1_f64 * t57060 + 0.181155e1_f64 * t57063 + 0.247573125e0_f64 * t57066 + 0.6189328125e-1_f64 * t57069 - 0.3883875e1_f64 * t57071 - 0.485484375e1_f64 * t57073 + 0.16504875e0_f64 * t57100 + 0.11651625e2_f64 * t57102 - 0.51785e1_f64 * t57104 + 0.258925e1_f64 * t57106;
    t57179
}
