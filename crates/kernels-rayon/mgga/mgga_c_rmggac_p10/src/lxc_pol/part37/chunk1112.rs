//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1112/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1112(t352: f64, t5148: f64, t76372: f64, t78114: f64, t78115: f64, t78116: f64, t78117: f64, t78119: f64, t78120: f64, t78189: f64, t78194: f64, t78199: f64, t78201: f64, t78203: f64, t78205: f64, t80452: f64) -> f64 {
    let t80489 = -t76372 + t78114 + t78115 + t78116 - t78117 - t78119 - t78120 - t78189 - t78194 - 0.11974241701863808564e0_f64 * t5148 * t80452 * t352 + t78199 + t78201 - t78203 + t78205;
    t80489
}
