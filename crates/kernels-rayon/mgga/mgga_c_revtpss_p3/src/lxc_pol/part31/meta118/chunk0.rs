//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 678/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk678(t251: f64, t836: f64, t231: f64, t2783: f64, t2782: f64, t233: f64, t860: f64, t869: f64, t689: f64, t136: f64, t2457: f64, t2710: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2784 = t251 * t836;
    let t2786 = t2783 * t2784 * t231;
    let t2787 = t2782 * t2786;
    let t2789 = t233 * t860;
    let t2790 = t869 * t2789;
    let t2791 = t689 * t2790;
    let t2793 = t251 * t136;
    let t2796 = 0.11565819519348392139e-2_f64 * t2710 * t2793 * t2457;
    (t2786, t2787, t2789, t2790, t2791, t2793, t2796)
}
