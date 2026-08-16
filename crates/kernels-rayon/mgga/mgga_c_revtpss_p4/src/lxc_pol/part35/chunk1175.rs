//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1175/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1175(t110541: f64, t25411: f64, t110275: f64, t93281: f64, t6049: f64, t689: f64, t7384: f64, t1580: f64, t28447: f64, t110502: f64, t25387: f64, t18797: f64, t26497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110544 = t25411 * t110541;
    let t110572 = t93281 * t110275;
    let t110584 = t689 * t7384 * t6049;
    let t110591 = t689 * t28447 * t1580;
    let t110600 = t25387 * t110502;
    let t110613 = t26497 * t18797;
    (t110544, t110572, t110584, t110591, t110600, t110613)
}
