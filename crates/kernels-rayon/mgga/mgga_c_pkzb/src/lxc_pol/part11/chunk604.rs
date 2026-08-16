//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 604/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk604(t27: f64, t3333: f64, t23: f64, t28: f64, t3315: f64, t3319: f64, t3324: f64, t3330: f64, t7: f64, t980: f64, t984: f64) -> (f64, f64) {
    let t3334 = t27 * t3333;
    let t3337 = 10.0_f64 / 9.0_f64 * t7 * t3315 + 5.0_f64 / 3.0_f64 * t7 * t3319 + 88.0_f64 / 9.0_f64 * t3324 * t28 - 80.0_f64 / 9.0_f64 * t980 * t984 + 10.0_f64 / 9.0_f64 * t23 * t3330 + 5.0_f64 / 3.0_f64 * t23 * t3334;
    (t3334, t3337)
}
