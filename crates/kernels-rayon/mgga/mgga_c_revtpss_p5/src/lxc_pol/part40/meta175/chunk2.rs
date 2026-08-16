//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 769/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk769(t30: f64, t33: f64, t2630: f64, t3869: f64, t1337: f64, t2619: f64, t514: f64, t1344: f64, t2257: f64, t3834: f64, t517: f64, t1348: f64, t3351: f64, t3842: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3871 = 0.10843581300301739842e-1_f64 * t3869 * t2630;
    let t3873 = 0.24415263074675393405e-3_f64 * t1337 * t2619;
    let t3874 = 1.0_f64 / t514;
    let t3880 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t3874 * t3834 + 2.0_f64 / 3.0_f64 * t1344 * t2257);
    let t3881 = 1.0_f64 / t517;
    let t3887 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t3881 * t3842 + 2.0_f64 / 3.0_f64 * t1348 * t3351);
    (t3871, t3873, t3874, t3880, t3881, t3887)
}
