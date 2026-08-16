//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 726/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk726<F: Float>(t1126: F, t1130: F, t1129: F, t418: F, t408: F, t406: F, t409: F, t3356: F, t281: F, t2902: F, t414: F, t1146: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3379 = t1126 * t1130;
    let t3382 = t1129 * t418;
    let t3383 = F::cast_from(1.0_f64) / t3382;
    let t3384 = t408 * t3383;
    let t3390 = F::cast_from(1.0_f64) / t409 / t406;
    let t3394 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3356;
    let t3402 = F::cast_from(0.39862222222222222223e0_f64) * t3356;
    let t3407 = F::cast_from(1.0_f64)/F::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = F::cast_from(0.13692777777777777778e0_f64) * t3413;
    let t3415 = t698 * t1146;
    (t3379, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3415)
}
