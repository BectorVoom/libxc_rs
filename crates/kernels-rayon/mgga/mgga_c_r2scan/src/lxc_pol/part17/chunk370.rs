//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 370/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk370(t1541: f64, t95: f64, t498: f64, t792: f64, t282: f64, t283: f64) -> (f64, f64, f64, f64) {
    let t1542 = t95 * t1541;
    let t1556 = t498 * t792;
    let t1559 = t282 * t282;
    let t1561 = 1.0_f64 / t283 / t1559;
    (t1542, t1556, t1559, t1561)
}
