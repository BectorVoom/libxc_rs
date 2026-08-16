//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 949/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk949(t1102: f64, t672: f64, t1098: f64, t140: f64, t3043: f64, t3039: f64, t1127: f64, t650: f64, t1015: f64, t242: f64, t1125: f64, t2850: f64, t3090: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9657 = t672 * t1102;
    let t9658 = t1098 * t9657;
    let t9660 = t140 * t3043;
    let t9661 = t1098 * t9660;
    let t9663 = t140 * t3039;
    let t9664 = t1098 * t9663;
    let t9666 = t650 * t1127;
    let t9668 = t242 * t9666 * t1015;
    let t9669 = t1125 * t9668;
    let t9672 = t242 * t3090 * t2850;
    (t9658, t9661, t9664, t9666, t9669, t9672)
}
