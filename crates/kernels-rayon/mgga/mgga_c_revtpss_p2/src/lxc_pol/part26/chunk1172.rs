//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1172/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1172(t26485: f64, t93342: f64, t10509: f64, t26481: f64, t25387: f64, t11015: f64, t7388: f64, t212: f64, t26473: f64, t689: f64, t780: f64, t26474: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95624 = t93342 * t26485;
    let t95628 = t26481 * t10509;
    let t95629 = t25387 * t95628;
    let t95632 = 0.30356481678079769392e-1_f64 * t7388 * t11015;
    let t95635 = t689 * t212 * t26473 * t780;
    let t95644 = t26474 * t72 * t686;
    (t95624, t95628, t95629, t95632, t95635, t95644)
}
