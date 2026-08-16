//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1034/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1034(t11585: f64, t2168: f64, t11340: f64, t824: f64, t905: f64, t2119: f64, t3916: f64, t2124: f64, t11542: f64, t11548: f64, t11554: f64, t11560: f64, t11566: f64, t11568: f64, t11570: f64, t11573: f64, t11578: f64, t11581: f64, t2277: f64, t2312: f64, t3247: f64, t6685: f64, t902: f64) -> (f64, f64, f64, f64, f64) {
    let t11587 = t2168 * t11585 / 16.0_f64;
    let t11588 = t11340 * t824;
    let t11589 = t905 * t11588;
    let t11592 = t3916 * t2119;
    let t11594 = t11592 * t2124 / 96.0_f64;
    let t11595 = t6685 * t11542 / 256.0_f64 + t2277 * t11548 / 768.0_f64 - t2312 * t11554 / 96.0_f64 - t2277 * t11560 / 768.0_f64 - t11566 - t11568 - t11570 + t2312 * t11573 / 384.0_f64 + t3247 * t11578 / 256.0_f64 - 7.0_f64 / 2304.0_f64 * t11581 - t11587 + t902 * t11589 / 1536.0_f64 - t11594;
    (t11587, t11588, t11589, t11594, t11595)
}
