//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1290/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1290(t1235: f64, t60706: f64, t18450: f64, t3334: f64, t3329: f64, t159: f64, t7091: f64, t1695: f64, t510: f64, t527: f64, t5543: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60707 = t60706 * t1235;
    let t60709 = t18450 * t3334;
    let t60713 = t18450 * t3329;
    let t60720 = t7091 * t159;
    let t60722 = t60720 * t510 * t1695;
    let t60724 = t5543 * t527;
    let t60725 = t60724 * t3247;
    (t60707, t60709, t60713, t60720, t60722, t60725)
}
