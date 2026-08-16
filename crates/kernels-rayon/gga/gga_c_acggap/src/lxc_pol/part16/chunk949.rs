//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 949/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk949(t7987: f64, t9054: f64, t29997: f64, t7942: f64, t8406: f64, t525: f64, t847: f64, t7932: f64, t7963: f64, t2138: f64, t2147: f64, t322: f64, t8436: f64) -> (f64, f64, f64, f64) {
    let t33783 = 0.34694512752820797848e1_f64 * t7987 * t9054;
    let t33786 = 0.17347256376410398924e1_f64 * t7942 * t29997 * t8406;
    let t33787 = t525 * t847;
    let t33789 = t7963 * t7932 * t33787;
    let t33794 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t8436 * t322;
    (t33783, t33786, t33789, t33794)
}
