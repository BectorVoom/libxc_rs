//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 171/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk171(t236: f64, t527: f64, t242: f64, t211: f64, t512: f64, t525: f64) -> (f64, f64) {
    let t528 = t236 * t527;
    let t529 = t528 * t242;
    let t532 = t211 * t512 / 96.0_f64 + t525 * t529 / 3072.0_f64;
    (t529, t532)
}
