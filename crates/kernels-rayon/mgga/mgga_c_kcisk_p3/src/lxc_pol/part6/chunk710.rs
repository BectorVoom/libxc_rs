//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 710/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk710(t12453: f64, t12649: f64, t190: f64, t207: f64, t206: f64, t9355: f64, t1039: f64, t3233: f64, t116: f64, t3241: f64, t3174: f64, t9345: f64) -> (f64, f64, f64, f64, f64) {
    let t12650 = t12453 + t12649;
    let t12651 = t12650 * t190;
    let t12652 = t12651 * t207;
    let t12654 = t206 * t9355;
    let t12656 = t3233 * t1039;
    let t12658 = t3241 * t116;
    let t12659 = t9345 * t3174;
    (t12652, t12654, t12656, t12658, t12659)
}
