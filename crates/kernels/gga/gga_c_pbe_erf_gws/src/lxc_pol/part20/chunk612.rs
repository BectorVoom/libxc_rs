//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 612/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk612<F: Float>(t186: F, t3444: F, t185: F, t2790: F, t997: F, t198: F, t3345: F, t561: F, t1017: F, t1803: F, t225: F, t3379: F, t1780: F, t231: F, t3401: F, t3405: F, t3409: F, t3413: F, t3417: F, t3419: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3445 = t186 * t3444;
    let t3447 = 2.0 / 15.0 * t185 * t3445;
    let t3449 = 8.0 / 15.0 * t2790 * t997;
    let t3450 = t198 * t3345;
    let t3451 = t186 * t3450;
    let t3453 = 4.0 / 15.0 * t561 * t3451;
    let t3454 = t1017 * t1017;
    let t3455 = t1803 * t3454;
    let t3456 = t186 * t3455;
    let t3458 = 4.0 / 15.0 * t185 * t3456;
    let t3459 = t3379 * t225;
    let t3462 = t3401 + t3405 - t3409 + t3413 - t3417 - t3419 - t3447 + t3449 + t3453 + t3458 - t1780 + 4.0 / 3.0 * t3459 * t231;
    (t3445, t3447, t3449, t3450, t3451, t3453, t3454, t3455, t3456, t3458, t3459, t3462)
}
