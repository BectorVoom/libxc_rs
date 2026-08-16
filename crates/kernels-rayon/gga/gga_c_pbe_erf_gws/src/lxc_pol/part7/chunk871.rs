//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 871/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk871(t11: f64, t16699: f64, t571: f64, t174: f64, t177: f64, t2200: f64, t395: f64, t4968: f64, t4973: f64, t16672: f64, t16677: f64, t16682: f64, t16686: f64, t16690: f64, t16693: f64, t16697: f64) -> (f64, f64, f64, f64, f64) {
    let t16701 = t11 * t571 * t16699;
    let t16704 = t174 * t2200 * t177;
    let t16705 = 0.19591358024691358025e-1_f64 * t16704;
    let t16706 = t395 * t4968;
    let t16708 = t395 * t4973;
    let t16710 = 0.45340000000000000001e-1_f64 * t16672 - 0.45340000000000000002e-1_f64 * t16677 + 0.37783333333333333335e-2_f64 * t16682 + 0.5037777777777777778e-2_f64 * t16686 - 0.4534e-1_f64 * t16690 + 0.6801e-1_f64 * t16693 - 0.11335e-1_f64 * t16697 - 0.15113333333333333333e-1_f64 * t16701 - t16705 - 0.15113333333333333333e-1_f64 * t16706 + 0.15113333333333333333e-1_f64 * t16708;
    (t16701, t16704, t16706, t16708, t16710)
}
