//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2047/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2047(t27668: f64, t995: f64, t25610: f64, t25460: f64, t3057: f64, t25698: f64, t378: f64, t8521: f64, t25705: f64, t3336: f64, t11108: f64, t7177: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94080 = t995 * t27668;
    let t94085 = t25610 * t27668;
    let t94095 = t3057 * t25460;
    let t94121 = t25698 * t378;
    let t94122 = t94121 * t8521;
    let t94138 = t25705 * t3336;
    let t94142 = t7177 * t11108;
    (t94080, t94085, t94095, t94122, t94138, t94142)
}
