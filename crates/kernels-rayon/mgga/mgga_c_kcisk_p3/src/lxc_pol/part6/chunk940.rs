//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 940/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk940(t29402: f64, t29539: f64, t29583: f64, t29625: f64, t1908: f64, t2604: f64, t9108: f64, t5400: f64, t12042: f64, t15989: f64, t16389: f64, t22564: f64, t22575: f64, t22583: f64, t22698: f64, t22705: f64, t22707: f64, t28362: f64, t28379: f64, t28387: f64, t28394: f64, t28404: f64) -> (f64, f64, f64, f64) {
    let t29627 = t29402 + t29539 + t29583 + t29625;
    let t29628 = t1908 * t29627;
    let t29636 = t9108 * t2604;
    let t29637 = t29636 * t5400;
    let t29653 = -0.68863333333333333332e0_f64 * t15989 + 0.94674375e0_f64 * t28362 + 0.3529725e1_f64 * t28394 - t12042 - 0.34731666666666666667e0_f64 * t16389 + 0.69463333333333333335e-1_f64 * t22698 + 0.34431666666666666666e0_f64 * t22564 - 0.103295e1_f64 * t22575 + 0.51647499999999999999e0_f64 * t22583 - 0.41678000000000000001e0_f64 * t22705 + 0.20839e0_f64 * t22707 + 0.62517e0_f64 * t28404 - 0.103295e1_f64 * t28379 + 0.309885e1_f64 * t28387;
    (t29628, t29636, t29637, t29653)
}
