//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 845/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk845<F: Float>(t2306: F, t4383: F, t4395: F, t19775: F, t824: F, t2169: F, t2200: F, t329: F, t2079: F, t19561: F, t6094: F, t825: F, t2365: F, t6472: F, t2409: F, t375: F, t6125: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19894 = t2306 * t4383;
    let t19898 = t4395 * t4383;
    let t19905 = t824 * t19775;
    let t20091 = t329 * t2200 * t2169;
    let t20133 = t2079 * t2079;
    let t20137 = t19561 * t6094;
    let t20138 = t20137 * t825;
    let t20142 = t6472 * t2365;
    let t20154 = t2169 * t2409;
    let t20173 = 1.0 / t6125 / t375;
    (t19894, t19898, t19905, t20091, t20133, t20137, t20138, t20142, t20154, t20173)
}
