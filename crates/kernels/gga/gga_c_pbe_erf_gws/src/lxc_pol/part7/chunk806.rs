//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 806/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk806<F: Float>(t1403: F, t1407: F, t4951: F, t11: F, t4949: F, t1243: F, t1766: F, t395: F, t4959: F, t4964: F, t2704: F, t574: F, t1770: F, t1760: F, t4953: F, t4977: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16718 = t4951 * t1403 * t1407;
    let t16720 = t11 * t4949 * t16718;
    let t16722 = t1243 * t1766;
    let t16724 = t395 * t4959;
    let t16726 = t395 * t4964;
    let t16728 = t2704 * t574;
    let t16730 = t1243 * t1770;
    let t16732 = t1243 * t1760;
    let t16734 = t395 * t4953;
    let t16736 = t395 * t4977;
    (t16718, t16720, t16722, t16724, t16726, t16728, t16730, t16732, t16734, t16736)
}
