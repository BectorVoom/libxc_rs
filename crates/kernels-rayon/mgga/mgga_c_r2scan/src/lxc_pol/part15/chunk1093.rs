//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1093/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1093(t10680: f64, t10681: f64, t10683: f64, t1375: f64, t10977: f64, t10981: f64, t37368: f64, t3436: f64, t6876: f64, t122: f64, t158: f64, t166: f64, t2315: f64, t3434: f64) -> (f64, f64, f64) {
    let t38308 = t10680 * t10681 * t1375 * t10683;
    let t38311 = t37368 * t10977 * t10981;
    let t38312 = 0.65053455985619242968e-4_f64 * t38311;
    let t38317 = t3436 * t6876;
    let t38322 = t3434 * t38317 * t158 * t166 * t2315 * t122;
    (t38308, t38312, t38322)
}
