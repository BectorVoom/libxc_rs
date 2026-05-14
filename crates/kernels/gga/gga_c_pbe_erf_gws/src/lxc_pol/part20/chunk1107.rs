//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1107/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1107<F: Float>(t54301: F, t1154: F, t51387: F, t14046: F, t3184: F, t3148: F, t14023: F, t14548: F, t863: F, t14058: F, t3279: F, t1158: F, t51395: F, t3268: F, t1140: F, t14083: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54302 = 7.0 / 576.0 * t54301;
    let t54305 = t51387 * t1154;
    let t54319 = t14046 * t3184;
    let t54320 = 7.0 / 72.0 * t54319;
    let t54322 = t14046 * t3148;
    let t54323 = 7.0 / 72.0 * t54322;
    let t54329 = t863 * t14023 * t14548;
    let t54330 = 7.0 / 24.0 * t54329;
    let t54344 = t14058 * t3279;
    let t54345 = 35.0 / 288.0 * t54344;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    let t54355 = 7.0 / 288.0 * t54354;
    let t54356 = t14083 * t1140;
    (t54302, t54305, t54320, t54323, t54330, t54345, t54352, t54355, t54356)
}
