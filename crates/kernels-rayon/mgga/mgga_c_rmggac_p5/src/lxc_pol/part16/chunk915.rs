//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 915/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk915(t25441: f64, t9948: f64, t202: f64, t461: f64, t6067: f64, t674: f64, t678: f64, t1763: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t875: f64) -> (f64, f64, f64) {
    let t45226 = t25441 * t9948;
    let t45234 = t6067 * t202 * t461 * t674 * t678;
    let t45240 = t1970 * t1971 * t875 * t1763 * t476 * t209;
    (t45226, t45234, t45240)
}
