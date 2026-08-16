//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1874/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1874(t25375: f64, t95765: f64, t25411: f64, t95597: f64, t93170: f64, t95746: f64, t26446: f64, t689: f64, t887: f64, t26481: f64, t2724: f64, t676: f64) -> (f64, f64, f64, f64, f64) {
    let t95832 = t25375 * t95765;
    let t95834 = t25411 * t95597;
    let t95836 = t93170 * t95746;
    let t95847 = t689 * t26446 * t887;
    let t95854 = t26481 * t676 * t2724;
    (t95832, t95834, t95836, t95847, t95854)
}
