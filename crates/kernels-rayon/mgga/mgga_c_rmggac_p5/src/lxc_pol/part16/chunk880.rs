//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 880/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk880(t9219: f64, t9223: f64, t9225: f64, t9229: f64, t9236: f64, t9675: f64, t9678: f64, t1970: f64, t1971: f64, t236: f64, t6149: f64, t6113: f64, t7365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44493 = 0.5107751987195740728e-4_f64 * t9219;
    let t44494 = 0.212822999466489197e-4_f64 * t9223;
    let t44495 = 0.17961362552795712846e0_f64 * t9225;
    let t44496 = 0.11974241701863808564e0_f64 * t9229;
    let t44498 = 0.1702583995731913576e-4_f64 * t9236;
    let t44499 = 0.4726e1_f64 * t9675;
    let t44500 = 2.0_f64 * t9678;
    let t44580 = t1970 * t1971 * t236 * t6149;
    let t44584 = t7365 * t1971 * t236 * t6113;
    (t44493, t44494, t44495, t44496, t44498, t44499, t44500, t44580, t44584)
}
