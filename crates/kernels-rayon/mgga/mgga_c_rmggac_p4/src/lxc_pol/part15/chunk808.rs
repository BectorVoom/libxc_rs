//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 808/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk808(t290: f64, t9030: f64, t118: f64, t128: f64, t1494: f64, t1986: f64, t209: f64, t1550: f64, t5144: f64, t7778: f64, t5267: f64, t903: f64) -> (f64, f64, f64, f64) {
    let t39507 = t290 * t9030;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39528 = t1550 * t7778 * t5144;
    let t39529 = 0.15965655602485078085e0_f64 * t39528;
    let t39535 = t903 * t7778 * t5267;
    (t39507, t39513, t39529, t39535)
}
