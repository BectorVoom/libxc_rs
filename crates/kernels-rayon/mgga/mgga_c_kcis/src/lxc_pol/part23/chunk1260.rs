//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1260/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1260(t7908: f64, t98072: f64, t15879: f64, t4160: f64, t98530: f64, t1014: f64, t28429: f64, t16991: f64, t6176: f64, t7899: f64, t28531: f64, t1466: f64, t5870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98587 = 0.15445601851851851852e-3_f64 * t7908 * t98072;
    let t98593 = t4160 * t98530 * t15879;
    let t98597 = t1014 * t28429;
    let t98598 = 0.33163888888888888888e-2_f64 * t98597;
    let t98600 = t6176 * t7899 * t16991;
    let t98603 = t1014 * t28531;
    let t98604 = 0.33163888888888888888e-2_f64 * t98603;
    let t98607 = t5870 * t1466;
    (t98587, t98593, t98597, t98598, t98600, t98603, t98604, t98607)
}
