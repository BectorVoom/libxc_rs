//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 362/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk362(t382: f64, t14: f64, t31: f64, t1467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1481 = t382 * t382;
    let t1482 = 1.0_f64 / t1481;
    let t1483 = t14 * t1482;
    let t1484 = t31 * t31;
    let t1485 = 1.0_f64 / t1484;
    let t1486 = t1467 * t1485;
    let t1487 = t1483 * t1486;
    let t1488 = 0.16081979498692535067e2_f64 * t1487;
    (t1481, t1482, t1483, t1484, t1485, t1486, t1488)
}
