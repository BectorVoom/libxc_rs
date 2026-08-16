//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1031/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1031(t11557: f64, t820: f64, t9441: f64, t3257: f64, t346: f64, t3747: f64, t1114: f64, t2150: f64, t3134: f64, t9108: f64, t9111: f64, t3757: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11558 = t11557 * t820;
    let t11559 = t9441 * t11558;
    let t11560 = t3257 * t11559;
    let t11563 = t3747 * t346;
    let t11564 = t1114 * t11563;
    let t11566 = t11564 * t2150 / 48.0_f64;
    let t11568 = t9108 * t3134 / 48.0_f64;
    let t11570 = t9111 * t3134 / 48.0_f64;
    let t11571 = t3757 * t810;
    (t11559, t11560, t11566, t11568, t11570, t11571)
}
