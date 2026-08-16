//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2870/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2870(t45186: f64, t439: f64, t3522: f64, t3444: f64, t3451: f64, t1156: f64, t12428: f64, t43813: f64, t1209: f64, t13126: f64, t17708: f64, t1203: f64, t12626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45187 = 1.0_f64 / t45186;
    let t45188 = t439 * t45187;
    let t45189 = t3522 * t3522;
    let t45190 = 1.0_f64 / t45189;
    let t45194 = t3444 * t3451;
    let t45197 = t1156 * t12428;
    let t45232 = 0.17757530864197530864e0_f64 * t43813;
    let t45371 = t1209 * t13126 * t17708;
    let t45384 = t1203 * t12626;
    (t45187, t45188, t45190, t45194, t45197, t45232, t45371, t45384)
}
