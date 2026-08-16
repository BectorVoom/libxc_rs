//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1250/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1250(t1561: f64, t2885: f64, t2860: f64, t13550: f64, t13563: f64, t13644: f64, t13602: f64, t4446: f64, t942: f64, t1573: f64, t2929: f64, t13566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14271 = t1561 * t2885;
    let t14276 = t1561 * t2860;
    let t14287 = 0.27785333333333333334e0_f64 * t13550;
    let t14291 = 0.22954444444444444444e0_f64 * t13563;
    let t14321 = 0.13892666666666666667e0_f64 * t13644;
    let t14324 = 0.34431666666666666666e0_f64 * t13602;
    let t14332 = t4446 * t942;
    let t14337 = t1573 * t2929;
    let t14352 = 0.41203703703703703704e-2_f64 * t13563;
    let t14353 = 0.12361111111111111111e-1_f64 * t13566;
    (t14271, t14276, t14287, t14291, t14321, t14324, t14332, t14337, t14352, t14353)
}
