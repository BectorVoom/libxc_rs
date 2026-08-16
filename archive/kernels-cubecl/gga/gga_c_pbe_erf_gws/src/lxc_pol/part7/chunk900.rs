//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 900/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk900<F: Float>(t11: F, t17018: F, t625: F, t1416: F, t1692: F, t1243: F, t1699: F, t395: F, t5074: F, t5077: F, t5071: F, t5068: F) -> (F, F, F, F, F, F, F, F) {
    let t17020 = t11 * t625 * t17018;
    let t17022 = t1692 * t1416;
    let t17024 = t11 * t625 * t17022;
    let t17026 = t1243 * t1699;
    let t17028 = t395 * t5074;
    let t17030 = t395 * t5077;
    let t17032 = t395 * t5071;
    let t17034 = t395 * t5068;
    (t17020, t17022, t17024, t17026, t17028, t17030, t17032, t17034)
}
