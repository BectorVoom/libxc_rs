//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1733/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1733(t1284: f64, t5216: f64, t1204: f64, t5477: f64, t17814: f64, t3783: f64, t3302: f64, t3588: f64, t471: f64, t5332: f64, t1269: f64, t3781: f64) -> (f64, f64, f64, f64, f64) {
    let t17861 = t5216 * t1284;
    let t17864 = t1204 * t5477;
    let t17869 = t17814 * t3783;
    let t17875 = t3302 * t3588 * t471;
    let t17876 = t5332 * t17875;
    let t17879 = t3781 * t1269;
    (t17861, t17864, t17869, t17876, t17879)
}
