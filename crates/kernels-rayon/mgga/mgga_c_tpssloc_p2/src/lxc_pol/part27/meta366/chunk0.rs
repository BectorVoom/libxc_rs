//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1503/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1503(t13546: f64, t908: f64, t136: f64, t4389: f64, t699: f64, t4386: f64, t10277: f64, t1409: f64, t2244: f64) -> (f64, f64, f64, f64, f64) {
    let t13547 = t908 * t13546;
    let t13548 = t136 * t13547;
    let t13550 = t699 * t4389;
    let t13551 = 0.21908444444444444444e0_f64 * t13550;
    let t13552 = t699 * t4386;
    let t13554 = t10277 * t1409;
    let t13555 = t13554 * t2244;
    (t13548, t13550, t13551, t13552, t13555)
}
