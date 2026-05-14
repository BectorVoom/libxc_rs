//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 793/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk793<F: Float>(t1730: F, t2737: F, t4957: F, t950: F, t1403: F, t1856: F, t2775: F, t401: F, t1407: F, t2560: F, t4951: F, t5264: F, t1663: F, t34: F, t418: F, t2554: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7324 = 8.0 / 15.0 * t1730 * t2737;
    let t7325 = t4957 * t950;
    let t7326 = t7325 * t1403;
    let t7327 = t1856 * t7326;
    let t7335 = 0.2962962962962962963e-2 * t401 * t2775;
    let t7336 = t2560 * t1407;
    let t7337 = t1856 * t7336;
    let t7340 = t4951 * t950;
    let t7341 = t7340 * t1403;
    let t7342 = t5264 * t7341;
    let t7345 = t1663 * t34;
    let t7346 = t7345 * t418;
    let t7347 = t1856 * t7346;
    let t7350 = t2554 * t1407;
    (t7324, t7326, t7327, t7335, t7336, t7337, t7341, t7342, t7346, t7347, t7350)
}
