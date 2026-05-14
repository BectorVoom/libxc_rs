//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 827/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk827<F: Float>(t1413: F, t1416: F, t5002: F, t11: F, t1691: F, t1642: F, t16986: F, t4373: F, t5028: F, t5063: F, t5089: F, t16973: F, t625: F, t1692: F, t1243: F, t1699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17001 = t5002 * t1413 * t1416;
    let t17003 = t11 * t1691 * t17001;
    let t17005 = t1642 * t16986;
    let t17007 = t11 * t1691 * t17005;
    let t17009 = t5028 * t4373;
    let t17011 = t11 * t1691 * t17009;
    let t17014 = t5063 * t1413 * t1416;
    let t17016 = t11 * t5089 * t17014;
    let t17018 = t5002 * t16973;
    let t17020 = t11 * t625 * t17018;
    let t17022 = t1692 * t1416;
    let t17024 = t11 * t625 * t17022;
    let t17026 = t1243 * t1699;
    (t17001, t17003, t17005, t17007, t17009, t17011, t17014, t17016, t17018, t17020, t17022, t17024, t17026)
}
