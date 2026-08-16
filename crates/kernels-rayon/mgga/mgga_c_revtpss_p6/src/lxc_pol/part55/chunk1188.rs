//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1188/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1188(t31845: f64, t33695: f64, t839: f64, t119837: f64, t14686: f64, t1559: f64, t120011: f64, t120016: f64, t1544: f64, t886: f64, t119792: f64, t828: f64, t855: f64) -> (f64, f64, f64, f64, f64) {
    let t126121 = t33695 * t31845;
    let t126122 = t126121 * t839;
    let t126133 = t14686 * t119837 * t1559;
    let t126134 = t120011 * t126133;
    let t126136 = t120016 * t126133;
    let t126138 = t1544 * t886;
    let t126141 = t119792 * t855 * t828 * t126138;
    (t126122, t126134, t126136, t126138, t126141)
}
