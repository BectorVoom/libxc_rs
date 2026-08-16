//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 521/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk521(t1933: f64, t333: f64, t1734: f64, t338: f64, t352: f64, t551: f64, t570: f64) -> (f64, f64, f64) {
    let t6504 = t1933 * t333;
    let t6507 = t338 * t1734;
    let t6508 = t6507 * t352;
    let t6522 = t551 * t570;
    (t6504, t6508, t6522)
}
