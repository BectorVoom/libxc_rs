//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1130/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1130(t3591: f64, t37972: f64, t10872: f64, t11736: f64, t1615: f64, t3320: f64, t783: f64, t978: f64, t261: f64, t3299: f64, t7291: f64, t3594: f64, t37736: f64) -> (f64, f64, f64, f64, f64) {
    let t39552 = t37972 * t3591;
    let t39554 = t10872 * t11736;
    let t39558 = t783 * t978 * t1615 * t3320;
    let t39561 = t3299 * t261 * t7291;
    let t39563 = t37736 * t3594;
    (t39552, t39554, t39558, t39561, t39563)
}
