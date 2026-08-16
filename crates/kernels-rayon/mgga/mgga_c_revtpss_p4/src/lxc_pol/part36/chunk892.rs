//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 892/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk892(t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t1260: f64, t5326: f64, t17376: f64, t3599: f64, t1285: f64, t17395: f64, t1781: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17569 = t5326 * t1260;
    let t17572 = t17376 * t3599;
    let t17605 = t1285 * t17395;
    let t17628 = t697 * t1781;
    (t17523, t17525, t17529, t17569, t17572, t17605, t17628)
}
