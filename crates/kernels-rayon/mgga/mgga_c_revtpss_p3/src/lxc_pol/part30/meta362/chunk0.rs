//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1388/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1388(t12295: f64, t1284: f64, t3552: f64, t1204: f64, t3766: f64, t3555: f64, t3754: f64, t1248: f64, t3153: f64, t3588: f64, t5464: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12678 = 0.25925925925925925926e-1_f64 * t12295;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12713 = t5464 * t3588;
    let t12717 = t3566 * t3754;
    (t12678, t12699, t12702, t12709, t12712, t12713, t12717)
}
