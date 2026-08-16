//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1053/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1053(t1679: f64, t1717: f64, t9097: f64, t1814: f64, t2122: f64, t33489: f64, t7942: f64, t8406: f64, t157: f64, t1914: f64, t406: f64, t33796: f64, t9030: f64) -> (f64, f64, f64, f64, f64) {
    let t38615 = t1679 * t9097 * t1717;
    let t38621 = t2122 * t1814;
    let t38631 = t7942 * t33489 * t8406;
    let t38635 = t1914 * t406 * t157;
    let t38639 = t33796 * t9030;
    (t38615, t38621, t38631, t38635, t38639)
}
