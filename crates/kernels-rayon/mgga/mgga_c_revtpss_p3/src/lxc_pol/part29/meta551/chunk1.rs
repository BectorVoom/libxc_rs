//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1889/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889(t10073: f64, t25937: f64, t7282: f64, t7506: f64, t26069: f64, t96255: f64, t2453: f64, t3908: f64, t7507: f64, t2435: f64, t26301: f64, t7289: f64, t96276: f64) -> (f64, f64, f64, f64, f64) {
    let t96398 = t10073 * t7282 * t25937 * t7506;
    let t96401 = 0.91399340044406952588e-2_f64 * t26069 * t96255;
    let t96403 = t2453 * t7507 * t3908;
    let t96410 = t2435 * t26301;
    let t96412 = t7289 * t96276;
    (t96398, t96401, t96403, t96410, t96412)
}
