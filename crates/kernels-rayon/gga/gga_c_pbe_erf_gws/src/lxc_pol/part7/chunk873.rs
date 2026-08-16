//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 873/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk873(t395: f64, t4964: f64, t2704: f64, t574: f64, t1243: f64, t1770: f64, t1760: f64, t4953: f64, t4977: f64, t5292: f64, t56: f64, t1662: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16726 = t395 * t4964;
    let t16728 = t2704 * t574;
    let t16730 = t1243 * t1770;
    let t16732 = t1243 * t1760;
    let t16734 = t395 * t4953;
    let t16736 = t395 * t4977;
    let t16738 = t56 * t5292;
    let t16739 = t1662 * t1662;
    (t16726, t16728, t16730, t16732, t16734, t16736, t16738, t16739)
}
