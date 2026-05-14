//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 455/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk455<F: Float>(t3450: F, t426: F, t3356: F, t3413: F, t1159: F, t434: F, t1178: F, t444: F, t439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3451 = 1.0 / t3450;
    let t3452 = t426 * t3451;
    let t3459 = 0.68863333333333333333e0 * t3356;
    let t3466 = 0.17365833333333333333e0 * t3413;
    let t3475 = t1159 * t1159;
    let t3476 = 1.0 / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    let t3479 = 1.0 / t3478;
    let t3483 = 0.12361111111111111111e-1 * t3356;
    let t3494 = t1178 * t444;
    let t3495 = 1.0 / t3494;
    let t3496 = t439 * t3495;
    let t3503 = 0.40256666666666666667e0 * t3356;
    let t3510 = 0.137975e0 * t3413;
    let t3519 = t1178 * t1178;
    let t3520 = 1.0 / t3519;
    (t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478, t3479, t3483, t3495, t3496, t3503, t3510, t3519, t3520)
}
