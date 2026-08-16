//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 943/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk943(t1082: f64, t11173: f64, t3298: f64, t989: f64, t3059: f64, t3291: f64, t4980: f64, t994: f64, t3151: f64, t999: f64, t3304: f64, t4995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12111 = t1082 * t11173;
    let t12116 = t989 * t3298;
    let t12119 = t3291 * t3059;
    let t12122 = t994 * t4980;
    let t12123 = t999 * t3151;
    let t12124 = t12123 * t3304;
    let t12127 = t994 * t4995;
    (t12111, t12116, t12119, t12122, t12123, t12124, t12127)
}
