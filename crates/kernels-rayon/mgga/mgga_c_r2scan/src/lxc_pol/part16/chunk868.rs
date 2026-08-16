//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 868/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk868(t1600: f64, t3064: f64, t3087: f64, t1632: f64, t3053: f64, t551: f64, t574: f64, t2620: f64, t2651: f64, t3090: f64, t1592: f64, t133: f64, t2892: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9221 = t1600 * t3064;
    let t9223 = t1600 * t3087;
    let t9226 = t551 * t1632 * t3053;
    let t9227 = t574 * t9226;
    let t9229 = t2651 * t2620;
    let t9232 = t551 * t1632 * t3090;
    let t9233 = t1592 * t9232;
    let t9235 = t133 * t2892;
    (t9221, t9223, t9227, t9229, t9233, t9235)
}
