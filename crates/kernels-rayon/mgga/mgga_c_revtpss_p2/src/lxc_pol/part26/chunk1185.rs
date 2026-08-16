//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1185/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1185(t25372: f64, t95536: f64, t92840: f64, t7398: f64, t822: f64, t25375: f64, t95765: f64, t25411: f64, t95597: f64, t93170: f64, t95746: f64, t26446: f64, t689: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95822 = t25372 * t95536;
    let t95823 = t95822 * t92840;
    let t95825 = t822 * t7398;
    let t95832 = t25375 * t95765;
    let t95834 = t25411 * t95597;
    let t95836 = t93170 * t95746;
    let t95847 = t689 * t26446 * t887;
    (t95823, t95825, t95832, t95834, t95836, t95847)
}
