//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 499/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk499(t1551: f64, t1554: f64, t1556: f64, t1562: f64, t1563: f64, t2259: f64, t285: f64, t495: f64, t499: f64) -> f64 {
    let t2262 = t1551 * t285 + t1554 * t285 + t495 * t1556 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t1562 * t1563 + t499 * t2259 / 4.0_f64;
    t2262
}
