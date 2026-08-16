//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1396/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1396(t10981: f64, t22: f64, t868: f64, t886: f64, t10910: f64, t212: f64, t689: f64, t780: f64, t10988: f64, t2435: f64, t2445: f64, t9292: f64) -> (f64, f64, f64, f64) {
    let t40978 = t10981 * t868 * t22 * t886;
    let t40982 = t689 * t212 * t10910 * t780;
    let t40986 = t2435 * t10988;
    let t40988 = t9292 * t2445;
    (t40978, t40982, t40986, t40988)
}
