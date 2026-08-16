//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1367/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1367(t14054: f64, t3992: f64, t2661: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64) -> (f64, f64, f64, f64, f64) {
    let t14055 = t3992 * t14054;
    let t14057 = 0.57165357490759649296e-4_f64 * t2661 * t14055;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = 0.19514881078765566038e-1_f64 * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = 0.19514881078765566038e-1_f64 * t14082 * t1364;
    let t14085 = t1357 * t5775;
    (t14057, t14079, t14081, t14084, t14085)
}
