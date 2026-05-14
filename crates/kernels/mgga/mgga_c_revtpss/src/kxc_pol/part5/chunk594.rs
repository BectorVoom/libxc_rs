//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 594/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk594<F: Float>(t1126: F, t1130: F, t1129: F, t418: F, t408: F, t406: F, t409: F, t3356: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3379 = t1126 * t1130;
    let t3382 = t1129 * t418;
    let t3383 = 1.0 / t3382;
    let t3384 = t408 * t3383;
    let t3390 = 1.0 / t409 / t406;
    let t3394 = 4.0 / 9.0 * t3356;
    let t3402 = 0.39862222222222222223e0 * t3356;
    let t3407 = 1.0/f64::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0 * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    (t3379, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3415, t3417)
}
