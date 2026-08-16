//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1884/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1884(t26276: f64, t9285: f64, t25944: f64, t136: f64, t2457: f64, t7531: f64, t26069: f64, t7515: f64, t94879: f64, t26230: f64, t9685: f64, t25878: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96255 = t26276 * t9285;
    let t96257 = 0.68540937416128198417e-2_f64 * t25944 * t96255;
    let t96259 = t7531 * t136 * t2457;
    let t96260 = t26069 * t96259;
    let t96262 = t94879 * t7515;
    let t96264 = t26230 * t9685;
    let t96265 = t25878 * t96264;
    (t96255, t96257, t96259, t96260, t96262, t96264, t96265)
}
