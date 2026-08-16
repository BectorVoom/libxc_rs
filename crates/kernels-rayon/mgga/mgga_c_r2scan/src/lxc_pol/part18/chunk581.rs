//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 581/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk581(t1562: f64, t2538: f64, t285: f64, t3053: f64, t3056: f64, t3060: f64, t3229: f64, t499: f64, t921: f64) -> f64 {
    let t3232 = t3053 * t285 + t3056 * t285 + t921 * t2538 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t1562 * t3060 + t499 * t3229 / 4.0_f64;
    t3232
}
