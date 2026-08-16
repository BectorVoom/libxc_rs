//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1018/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1018(t1188: f64, t6518: f64, t3503: f64, t3510: f64, t5044: f64, t5093: f64, t6423: f64, t6427: f64, t6431: f64, t6443: f64, t6450: f64, t6456: f64, t6458: f64, t6462: f64, t6465: f64, t6468: f64) -> (f64, f64) {
    let t6519 = t6518 * t1188;
    let t6534 = -0.1294625e1_f64 * t6443 + 0.258925e1_f64 * t6450 + t3503 - 0.20128333333333333334e0_f64 * t5044 - 0.20128333333333333333e0_f64 * t6423 + 0.60385e0_f64 * t6427 + 0.301925e0_f64 * t6431 + 0.82524375e-1_f64 * t6456 + 0.16504875e0_f64 * t6458 + t3510 - 0.11038e0_f64 * t5093 - 0.27595e-1_f64 * t6462 + 0.16557e0_f64 * t6465 + 0.82785e-1_f64 * t6468;
    (t6519, t6534)
}
