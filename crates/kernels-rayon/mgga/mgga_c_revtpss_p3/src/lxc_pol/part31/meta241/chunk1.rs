//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1080/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1080(t1169: f64, t6486: f64, t3459: f64, t3466: f64, t5044: f64, t5093: f64, t6423: f64, t6427: f64, t6431: f64, t6443: f64, t6450: f64, t6456: f64, t6458: f64, t6462: f64, t6465: f64, t6468: f64) -> (f64, f64) {
    let t6487 = t6486 * t1169;
    let t6502 = -0.17648625e1_f64 * t6443 + 0.3529725e1_f64 * t6450 + t3459 - 0.34431666666666666666e0_f64 * t5044 - 0.34431666666666666667e0_f64 * t6423 + 0.103295e1_f64 * t6427 + 0.516475e0_f64 * t6431 + 0.31558125e0_f64 * t6456 + 0.6311625e0_f64 * t6458 + t3466 - 0.13892666666666666667e0_f64 * t5093 - 0.34731666666666666667e-1_f64 * t6462 + 0.20839e0_f64 * t6465 + 0.104195e0_f64 * t6468;
    (t6487, t6502)
}
