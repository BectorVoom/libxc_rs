//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 580/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk580(t1556: f64, t1562: f64, t2532: f64, t2533: f64, t2534: f64, t2538: f64, t2541: f64, t2847: f64, t495: f64, t499: f64, t921: f64) -> f64 {
    let t2850 = t2532 + t2533 * t2534 + t921 * t1556 / 4.0_f64 + t495 * t2538 / 4.0_f64 - 5.0_f64 / 16.0_f64 * t1562 * t2541 + t499 * t2847 / 4.0_f64;
    t2850
}
