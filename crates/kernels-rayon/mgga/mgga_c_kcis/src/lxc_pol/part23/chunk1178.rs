//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1178/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1178(t165: f64, t92213: f64, t92254: f64, t92292: f64, t92336: f64, t2538: f64, t2626: f64, t7630: f64, t26416: f64, t826: f64, t9275: f64, t26398: f64, t9279: f64) -> (f64, f64, f64, f64) {
    let t92339 = (t92213 + t92254 + t92292 + t92336) * t165;
    let t92344 = 6.0_f64 * t2538 * t7630 * t2626;
    let t92351 = 18.0_f64 * t9275 * t26416 * t826;
    let t92356 = 6.0_f64 * t26398 * t9279;
    (t92339, t92344, t92351, t92356)
}
