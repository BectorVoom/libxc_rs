//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 920/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk920(t30861: f64, t7495: f64, t7676: f64, t7720: f64, t2092: f64, t7630: f64, t2087: f64, t1160: f64, t30539: f64, t1167: f64, t151: f64, t2116: f64, t3668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31619 = t30861 * t7495;
    let t31625 = t7676 * t7720;
    let t31627 = t7630 * t2092;
    let t31629 = t7630 * t2087;
    let t31631 = t1160 * t30539;
    let t31632 = t31631 * t1167;
    let t31643 = t151 * t2116 * t3668;
    (t31619, t31625, t31627, t31629, t31631, t31632, t31643)
}
