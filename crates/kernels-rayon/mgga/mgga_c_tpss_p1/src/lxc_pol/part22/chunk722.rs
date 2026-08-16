//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 722/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk722(t57: f64, t1289: f64, t2232: f64, t3431: f64, t581: f64, t81: f64, t3581: f64, t162: f64, t187: f64, t2224: f64, t2281: f64, t2285: f64, t2351: f64, t2439: f64, t3546: f64, t3547: f64, t3548: f64, t3552: f64, t3553: f64, t3559: f64, t3562: f64, t3563: f64, t3568: f64, t3571: f64, t3574: f64, t750: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t3582 = t2232 * t1289;
    let t3588 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t3582 * t581 - 4.0_f64 / 3.0_f64 * t81 * t3431);
    let t3589 = t3581 + t3588;
    let t3590 = t3589 * t162;
    let t3592 = 0.19751673498613801407e-1_f64 * t3590 * t187;
    let t3593 = 3.0_f64 * t2439 * t3548 * t750 + 6.0_f64 * t3552 * t3553 * t750 + t2224 - t2281 - t2285 + t2351 + t3546 + t3547 - t3559 - t3562 - t3563 + t3568 + t3571 + t3574 + t3592;
    (t3582, t3589, t3590, t3592, t3593)
}
