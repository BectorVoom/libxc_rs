//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1201/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1201(t3493: f64, t983: f64, t11002: f64, t3269: f64, t3262: f64, t3465: f64, t39183: f64, t11621: f64, t3275: f64, t39040: f64, t3719: f64, t481: f64) -> (f64, f64, f64, f64) {
    let t41326 = t3493 * t983;
    let t41327 = t11002 * t41326;
    let t41329 = 5.0_f64 / 8.0_f64 * t3269 * t41327;
    let t41332 = 3.0_f64 / 2.0_f64 * t3262 * t3465 * t39183;
    let t41335 = 45.0_f64 / 32.0_f64 * t3275 * t39040 * t11621;
    let t41336 = t3719 * t481;
    (t41329, t41332, t41335, t41336)
}
