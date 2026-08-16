//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1122/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1122<F: Float>(t2079: F, t2081: F, t19561: F, t6094: F, t825: F, t2365: F, t6472: F, t6800: F, t833: F, t2169: F, t2409: F, t2376: F, t2410: F, t814: F) -> (F, F, F, F, F, F, F) {
    let t20133 = t2079 * t2079;
    let t20134 = t2081 * t2081;
    let t20135 = t20133 * t20134;
    let t20137 = t19561 * t6094;
    let t20138 = t20137 * t825;
    let t20142 = t6472 * t2365;
    let t20144 = t6800 * t20142 * t833;
    let t20154 = t2169 * t2409;
    let t20157 = t20154 * t2376 * t2410 * t814;
    (t20133, t20134, t20135, t20137, t20138, t20144, t20157)
}
