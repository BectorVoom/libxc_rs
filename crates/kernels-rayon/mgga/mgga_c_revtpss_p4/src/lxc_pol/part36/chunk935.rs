//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 935/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk935(t22061: f64, t3992: f64, t2661: f64, t221: f64, t6836: f64, t9921: f64, t3978: f64, t125: f64, t6816: f64, t6843: f64, t13848: f64, t6869: f64, t9818: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22062 = t3992 * t22061;
    let t22063 = t2661 * t22062;
    let t22068 = t9921 * t221 * t6836;
    let t22069 = t3978 * t22068;
    let t22074 = t125 * t6816;
    let t22079 = t125 * t6843;
    let t22102 = t9818 * t13848 * t6869;
    (t22063, t22068, t22069, t22074, t22079, t22102)
}
