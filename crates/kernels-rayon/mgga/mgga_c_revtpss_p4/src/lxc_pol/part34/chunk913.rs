//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 913/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk913(t125: f64, t6861: f64, t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64, t22025: f64, t543: f64, t3992: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22046 = t125 * t6861;
    let t22056 = t3979 * t221 * t6816;
    let t22057 = t3978 * t22056;
    let t22059 = t3989 * t6880;
    let t22061 = t22025 * t543;
    let t22062 = t3992 * t22061;
    let t22063 = t2661 * t22062;
    (t22046, t22056, t22057, t22059, t22061, t22063)
}
