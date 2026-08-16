//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 646/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk646(t11: f64, t5074: f64, t5038: f64, t625: f64, t174: f64, t205: f64, t838: f64, t1243: f64, t628: f64, t1703: f64, t395: f64, t1693: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5075 = t11 * t5074;
    let t5077 = t625 * t5038;
    let t5078 = t11 * t5077;
    let t5081 = t174 * t838 * t205;
    let t5082 = 0.11197407407407407407e0_f64 * t5081;
    let t5083 = t1243 * t628;
    let t5085 = t395 * t1703;
    let t5087 = t395 * t1693;
    (t5075, t5077, t5078, t5081, t5082, t5083, t5085, t5087)
}
