//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 733/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk733(t609: f64, t864: f64, t2132: f64, t7885: f64, t448: f64, t939: f64, t2130: f64, t862: f64) -> (f64, f64, f64, f64, f64) {
    let t7886 = t609 * t864;
    let t7887 = t2132 * t7886;
    let t7889 = 0.26020884564615598386e1_f64 * t7885 * t7887;
    let t7890 = t448 * t939;
    let t7896 = t862 * t2130;
    (t7886, t7887, t7889, t7890, t7896)
}
