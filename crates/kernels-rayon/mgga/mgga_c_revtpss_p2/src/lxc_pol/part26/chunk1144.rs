//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1144/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1144(t1412: f64, t1941: f64, t9750: f64, t25273: f64, t540: f64, t1372: f64, t2019: f64, t9951: f64, t2018: f64, t9646: f64, t9723: f64, t26014: f64, t2689: f64) -> (f64, f64, f64, f64, f64) {
    let t94516 = t1941 * t1412;
    let t94517 = t94516 * t9750;
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    let t94522 = t2019 * t9951;
    let t94525 = t9646 * t2018 * t9723;
    let t94527 = t2689 * t26014;
    (t94517, t94520, t94522, t94525, t94527)
}
