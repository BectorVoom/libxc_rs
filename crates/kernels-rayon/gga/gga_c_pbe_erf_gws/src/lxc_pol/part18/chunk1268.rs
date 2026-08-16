//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1268/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1268(t15386: f64, t321: f64, t15382: f64, t945: f64, t14767: f64, t2503: f64, t4002: f64, t9955: f64, t29260: f64, t3808: f64, t3972: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56057 = t321 * t15386;
    let t56059 = t15382 * t945;
    let t56060 = t321 * t56059;
    let t56061 = t14767 * t2503;
    let t56063 = t9955 * t4002;
    let t56067 = t3972 * t3975 * t3808 * t29260;
    (t56057, t56059, t56060, t56061, t56063, t56067)
}
