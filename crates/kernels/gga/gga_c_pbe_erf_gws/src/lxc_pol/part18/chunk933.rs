//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 933/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk933<F: Float>(t11583: F, t6523: F, t875: F, t2168: F, t11340: F, t824: F, t905: F, t2119: F, t3916: F, t2124: F, t11542: F, t11548: F, t11554: F, t11560: F, t11566: F, t11568: F, t11570: F, t11573: F, t11578: F, t11581: F, t2277: F, t2312: F, t3247: F, t6685: F, t902: F) -> (F, F, F, F, F, F) {
    let t11585 = t6523 * t11583 * t875;
    let t11587 = t2168 * t11585 / 16.0;
    let t11588 = t11340 * t824;
    let t11589 = t905 * t11588;
    let t11592 = t3916 * t2119;
    let t11594 = t11592 * t2124 / 96.0;
    let t11595 = t6685 * t11542 / 256.0 + t2277 * t11548 / 768.0 - t2312 * t11554 / 96.0 - t2277 * t11560 / 768.0 - t11566 - t11568 - t11570 + t2312 * t11573 / 384.0 + t3247 * t11578 / 256.0 - 7.0 / 2304.0 * t11581 - t11587 + t902 * t11589 / 1536.0 - t11594;
    (t11585, t11587, t11588, t11589, t11594, t11595)
}
