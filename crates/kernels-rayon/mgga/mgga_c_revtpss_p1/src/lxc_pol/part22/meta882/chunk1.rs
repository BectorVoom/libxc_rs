//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3056/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056(t14477: f64, t2435: f64, t14978: f64, t2465: f64, t686: f64, t72: f64, t14480: f64, t252: f64, t2782: f64, t2828: f64, t10073: f64, t14482: f64) -> (f64, f64, f64, f64) {
    let t51741 = t2435 * t14477;
    let t51746 = t2465 * t14978 * t72 * t686;
    let t51750 = t2782 * t252 * t14480 * t2828;
    let t51756 = t10073 * t14482;
    (t51741, t51746, t51750, t51756)
}
