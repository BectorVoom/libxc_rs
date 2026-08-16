//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 844/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk844(t1188: f64, t3497: f64, t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64) -> (f64, f64, f64, f64) {
    let t3498 = t3497 * t1188;
    let t3503 = 0.40256666666666666667e0_f64 * t3356;
    let t3510 = 0.137975e0_f64 * t3413;
    let t3515 = -0.1294625e1_f64 * t3392 + 0.258925e1_f64 * t3400 + t3503 - 0.20128333333333333334e0_f64 * t3358 - 0.20128333333333333333e0_f64 * t3365 + 0.60385e0_f64 * t3370 + 0.301925e0_f64 * t3374 + 0.82524375e-1_f64 * t3408 + 0.16504875e0_f64 * t3410 + t3510 - 0.11038e0_f64 * t3415 - 0.27595e-1_f64 * t3419 + 0.16557e0_f64 * t3422 + 0.82785e-1_f64 * t3425;
    (t3498, t3503, t3510, t3515)
}
