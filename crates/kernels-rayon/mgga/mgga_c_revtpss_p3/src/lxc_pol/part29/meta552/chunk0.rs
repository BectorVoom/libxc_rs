//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1890/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1890(t26265: f64, t9671: f64, t26230: f64, t94403: f64, t25904: f64, t4078: f64, t689: f64, t7492: f64, t94589: f64, t96279: f64, t25895: f64, t96239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96423 = t26265 * t9671;
    let t96431 = t26230 * t94403;
    let t96432 = t25904 * t96431;
    let t96437 = t689 * t7492 * t4078;
    let t96456 = t94589 * t96279;
    let t96458 = t25895 * t96239;
    (t96423, t96431, t96432, t96437, t96456, t96458)
}
