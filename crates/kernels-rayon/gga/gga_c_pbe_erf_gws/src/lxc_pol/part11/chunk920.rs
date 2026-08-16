//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 920/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk920(t1216: f64, t1314: f64, t470: f64, t4734: f64, t4737: f64, t1215: f64, t457: f64, t4619: f64, t1327: f64, t1333: f64, t1319: f64, t1322: f64, t18563: f64) -> (f64, f64, f64, f64) {
    let t18933 = 0.61523382126046769581e4_f64 * t470 * t4734 * t1216 * t4737 * t1314;
    let t18939 = 0.46785787179641632568e1_f64 * t470 * t1215 * t4619 * t457;
    let t18941 = 120.0_f64 * t1333 * t1327;
    let t18950 = 0.51947267698127589897e2_f64 * t470 * t1319 * t18563 * t1322;
    (t18933, t18939, t18941, t18950)
}
