//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 966/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk966(t36103: f64, t41150: f64, t41027: f64, t793: f64, t2350: f64, t26531: f64, t41035: f64, t797: f64, t41043: f64, t851: f64, t5204: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41189 = t36103 * t41150;
    let t41191 = t793 * t41027;
    let t41193 = t26531 * t2350;
    let t41195 = t797 * t41035;
    let t41197 = t851 * t41043;
    let t41209 = t649 * t5204;
    (t41189, t41191, t41193, t41195, t41197, t41209)
}
