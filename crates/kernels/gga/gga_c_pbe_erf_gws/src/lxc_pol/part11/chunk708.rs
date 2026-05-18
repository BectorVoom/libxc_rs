//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 708/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk708<F: Float>(t3361: F, t409: F, t414: F, t153: F, t3373: F, t542: F, t3488: F, t583: F, t1630: F, t3499: F, t639: F, t181: F, t995: F) -> (F, F, F, F, F, F, F) {
    let t10257 = t409 * t3361;
    let t10259 = t414 * t3361;
    let t10283 = t153 * t542 * t3373;
    let t10293 = t3488 * t583;
    let t10300 = t1630 * t3499;
    let t10301 = t639 * t10300;
    let t10325 = t995 * t181;
    (t10257, t10259, t10283, t10293, t10300, t10301, t10325)
}
