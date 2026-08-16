//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 917/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk917(t18150: f64, t713: f64, t193: f64, t89: f64, t16579: f64, t669: f64, t666: f64, t4934: f64, t7514: f64, t3717: f64, t3821: f64, t2336: f64, t4930: f64) -> (f64, f64, f64, f64, f64) {
    let t18151 = t18150 * t713;
    let t18153 = t89 * t193 * t18151;
    let t18155 = t669 * t16579;
    let t18157 = t89 * t666 * t18155;
    let t18159 = t7514 * t4934;
    let t18160 = t18159 * t713;
    let t18162 = t89 * t193 * t18160;
    let t18163 = t3717 * t3821;
    let t18165 = t89 * t193 * t18163;
    let t18168 = t89 * t2336 * t4930;
    (t18153, t18157, t18162, t18165, t18168)
}
