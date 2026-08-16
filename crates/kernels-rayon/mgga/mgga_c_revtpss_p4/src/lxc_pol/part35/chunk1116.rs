//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1116/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1116(t7262: f64, t820: f64, t844: f64, t2482: f64, t596: f64, t7269: f64, t25981: f64, t843: f64, t2681: f64, t533: f64, t816: f64, t92993: f64) -> (f64, f64, f64, f64, f64) {
    let t94429 = t820 * t7262 * t844;
    let t94443 = t2482 * t7269 * t596;
    let t94455 = t820 * t25981 * t843;
    let t94459 = t820 * t7262 * t2681;
    let t94471 = t92993 * t533 * t816;
    (t94429, t94443, t94455, t94459, t94471)
}
