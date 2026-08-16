//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 395/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk395(t3127: f64, t363: f64, t3037: f64, t3033: f64, t360: f64, t2770: f64, t2978: f64, t2775: f64, t976: f64, t221: f64, t2965: f64, t339: f64) -> (f64, f64, f64, f64, f64) {
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    let t3146 = t2978 * t2770;
    let t3151 = t976 * t2775;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / 432.0_f64;
    (t3130, t3131, t3146, t3151, t3160)
}
