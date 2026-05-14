//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 804/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk804<F: Float>(t1663: F, t16679: F, t11: F, t1758: F, t4360: F, t4962: F, t16669: F, t4957: F, t571: F, t16613: F, t1764: F, t4971: F, t174: F, t177: F, t2200: F, t395: F, t4968: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16680 = t1663 * t16679;
    let t16682 = t11 * t1758 * t16680;
    let t16684 = t4962 * t4360;
    let t16686 = t11 * t1758 * t16684;
    let t16688 = t4957 * t16669;
    let t16690 = t11 * t571 * t16688;
    let t16693 = t11 * t571 * t16613;
    let t16695 = t1764 * t16679;
    let t16697 = t11 * t571 * t16695;
    let t16699 = t4971 * t4360;
    let t16701 = t11 * t571 * t16699;
    let t16704 = t174 * t2200 * t177;
    let t16705 = 0.19591358024691358025e-1 * t16704;
    let t16706 = t395 * t4968;
    (t16680, t16682, t16684, t16686, t16688, t16690, t16693, t16695, t16697, t16699, t16701, t16704, t16705, t16706)
}
