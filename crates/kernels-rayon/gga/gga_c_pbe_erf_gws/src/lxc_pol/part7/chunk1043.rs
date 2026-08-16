//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1043/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1043(t1444: f64, t4816: f64, t1216: f64, t1314: f64, t470: f64, t4734: f64, t4737: f64, t1399: f64, t4805: f64, t1215: f64, t457: f64, t4619: f64) -> (f64, f64, f64, f64) {
    let t18927 = t4816 * t1444;
    let t18928 = 0.14649244029402527953e-2_f64 * t18927;
    let t18933 = 0.61523382126046769581e4_f64 * t470 * t4734 * t1216 * t4737 * t1314;
    let t18934 = t1399 * t4805;
    let t18935 = 0.1403573615389248977e2_f64 * t18934;
    let t18939 = 0.46785787179641632568e1_f64 * t470 * t1215 * t4619 * t457;
    (t18928, t18933, t18935, t18939)
}
