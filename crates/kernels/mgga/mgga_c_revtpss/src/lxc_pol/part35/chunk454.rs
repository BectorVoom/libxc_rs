//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 454/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk454<F: Float>(t1129: F, t418: F, t408: F, t406: F, t409: F, t3356: F, t281: F, t2902: F, t414: F, t1224: F, t240: F, t421: F, t1159: F, t431: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3382 = t1129 * t418;
    let t3383 = 1.0 / t3382;
    let t3384 = t408 * t3383;
    let t3390 = 1.0 / t409 / t406;
    let t3394 = 4.0 / 9.0 * t3356;
    let t3402 = 0.39862222222222222223e0 * t3356;
    let t3407 = 1.0/f64::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0 * t3413;
    let t3417 = t240 * t1224;
    let t3431 = t1129 * t1129;
    let t3432 = 1.0 / t3431;
    let t3433 = t408 * t3432;
    let t3434 = t421 * t421;
    let t3435 = 1.0 / t3434;
    let t3439 = 0.22831111111111111111e-1 * t3356;
    let t3450 = t1159 * t431;
    (t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3417, t3431, t3432, t3433, t3434, t3435, t3439, t3450)
}
