//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 694/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk694(t1169: f64, t3453: f64, t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64) -> (f64, f64, f64, f64) {
    let t3454 = t3453 * t1169;
    let t3459 = 0.68863333333333333333e0_f64 * t3356;
    let t3466 = 0.17365833333333333333e0_f64 * t3413;
    let t3471 = -0.17648625e1_f64 * t3392 + 0.3529725e1_f64 * t3400 + t3459 - 0.34431666666666666666e0_f64 * t3358 - 0.34431666666666666667e0_f64 * t3365 + 0.103295e1_f64 * t3370 + 0.516475e0_f64 * t3374 + 0.31558125e0_f64 * t3408 + 0.6311625e0_f64 * t3410 + t3466 - 0.13892666666666666667e0_f64 * t3415 - 0.34731666666666666667e-1_f64 * t3419 + 0.20839e0_f64 * t3422 + 0.104195e0_f64 * t3425;
    (t3454, t3459, t3466, t3471)
}
