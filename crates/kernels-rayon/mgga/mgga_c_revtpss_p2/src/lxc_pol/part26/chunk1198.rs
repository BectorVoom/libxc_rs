//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1198/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1198(t25894: f64, t96186: f64, t94398: f64, t122: f64, t72: f64, t7506: f64, t25900: f64, t25904: f64, t26231: f64, t94802: f64, t2435: f64, t26355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96187 = t25894 * t96186;
    let t96188 = t96187 * t94398;
    let t96191 = t7506 * t72 * t122;
    let t96192 = t96191 * t25900;
    let t96193 = t25904 * t96192;
    let t96195 = t94802 * t26231;
    let t96197 = t2435 * t26355;
    (t96188, t96191, t96192, t96193, t96195, t96197)
}
