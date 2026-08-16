//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1447/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1447(t2770: f64, t3966: f64, t10216: f64, t1409: f64, t2775: f64, t4389: f64, t699: f64, t4386: f64, t10277: f64, t4339: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13527 = t2770 * t3966;
    let t13536 = t10216 * t1409;
    let t13541 = t2775 * t3966;
    let t13550 = t699 * t4389;
    let t13551 = 0.21908444444444444444e0_f64 * t13550;
    let t13552 = t699 * t4386;
    let t13554 = t10277 * t1409;
    let t13563 = t690 * t4339;
    (t13527, t13536, t13541, t13550, t13551, t13552, t13554, t13563)
}
