//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2801/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2801(t51512: f64, t10872: f64, t40298: f64, t40303: f64, t40307: f64, t40311: f64, t40314: f64, t40316: f64, t40318: f64, t51498: f64, t51505: f64, t51507: f64, t820: f64) -> f64 {
    let t51513 = 0.39029762157531132076e-1_f64 * t51512;
    let t51515 = -0.39512695097613069591e1_f64 * t820 * t51498 * t10872 - 0.29272321618148349057e-1_f64 * t40298 - 0.16463622957338778996e-1_f64 * t51505 + 0.43902994552903410656e-1_f64 * t51507 - 0.21951497276451705329e-1_f64 * t40303 + 0.54878743191129263322e-2_f64 * t40307 - 0.54878743191129263322e-2_f64 * t40311 - t40314 + t40316 + t51513 + 0.33133632253434461091e-3_f64 * t40318;
    t51515
}
