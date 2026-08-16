//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1143/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1143(t10903: f64, t11770: f64, t2201: f64, t2719: f64, t3319: f64, t3320: f64, t2207: f64, t2526: f64, t10899: f64, t11764: f64, t10841: f64, t2842: f64) -> (f64, f64, f64, f64, f64) {
    let t39894 = t2201 * t10903 * t11770;
    let t39899 = t2201 * t3319 * t3320 * t2719;
    let t39903 = t2207 * t3319 * t3320 * t2526;
    let t39906 = t2207 * t10899 * t11764;
    let t39908 = t10841 * t2842;
    (t39894, t39899, t39903, t39906, t39908)
}
