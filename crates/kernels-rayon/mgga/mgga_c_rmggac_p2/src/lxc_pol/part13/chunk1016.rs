//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1016/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1016(t357: f64, t577: f64, t7933: f64, t7934: f64, t132: f64, t1412: f64, t36912: f64, t9082: f64, t36935: f64, t202: f64, t461: f64, t5527: f64, t674: f64, t678: f64) -> (f64, f64, f64, f64, f64) {
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42246 = t7933 * t7934 * t1412 * t132;
    let t42248 = t36912 * t9082;
    let t42250 = t36935 * t9082;
    let t42255 = t5527 * t202 * t461 * t674 * t678;
    (t42242, t42246, t42248, t42250, t42255)
}
