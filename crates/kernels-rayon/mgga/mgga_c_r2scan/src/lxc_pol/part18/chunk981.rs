//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 981/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk981(t11760: f64, t3320: f64, t783: f64, t910: f64, t2207: f64, t3319: f64, t10856: f64, t2605: f64, t938: f64, t2201: f64, t2842: f64, t3281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11762 = t783 * t11760 * t3320;
    let t11764 = t3320 * t910;
    let t11766 = t2207 * t3319 * t11764;
    let t11768 = t10856 * t2605;
    let t11770 = t3320 * t938;
    let t11772 = t2201 * t3319 * t11770;
    let t11774 = t3281 * t2842;
    (t11762, t11764, t11766, t11768, t11770, t11772, t11774)
}
