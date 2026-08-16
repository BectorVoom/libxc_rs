//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1227/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1227(t25273: f64, t540: f64, t2019: f64, t9951: f64, t2018: f64, t9646: f64, t9723: f64, t2681: f64, t7269: f64, t820: f64, t240: f64, t25981: f64) -> (f64, f64, f64, f64, f64) {
    let t94519 = t25273 * t540;
    let t94522 = t2019 * t9951;
    let t94523 = 0.7558530601555998074e-1_f64 * t94522;
    let t94525 = t9646 * t2018 * t9723;
    let t94526 = 0.4016411544023718989e-6_f64 * t94525;
    let t94545 = t820 * t7269 * t2681;
    let t94550 = t25981 * t240;
    (t94519, t94523, t94526, t94545, t94550)
}
