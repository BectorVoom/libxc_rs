//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 956/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk956<F: Float>(t1557: F, t19238: F, t128: F, t1508: F, t19: F, t19253: F, t156: F, t5798: F, t496: F, t1504: F, t10: F, t5825: F, t16423: F, t506: F, t119: F, t331: F) -> (F, F, F, F, F, F, F, F) {
    let t19259 = t1557 * t19238;
    let t19263 = t1508 * t128 * t19 * t19253;
    let t19264 = 0.38973666666666666666e1 * t19263;
    let t19265 = t156 * t5798;
    let t19266 = t496 * t19265;
    let t19268 = t1504 * t1504;
    let t19270 = t10 * t5825 * t19268;
    let t19274 = t10 * t506 * t16423;
    let t19278 = t119 * t331 * t1504;
    (t19259, t19264, t19265, t19266, t19268, t19270, t19274, t19278)
}
