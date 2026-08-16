//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1067/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1067(t1615: f64, t3320: f64, t774: f64, t783: f64, t1575: f64, t2096: f64, t571: f64, t10710: f64, t20665: f64, t3342: f64, t572: f64, t546: f64) -> (f64, f64, f64, f64) {
    let t37707 = t783 * t774 * t1615 * t3320;
    let t37712 = t571 * t1575 * t2096;
    let t37714 = t37712 * t10710 * t20665;
    let t37716 = t572 * t3342;
    let t37717 = t546 * t37716;
    (t37707, t37714, t37716, t37717)
}
