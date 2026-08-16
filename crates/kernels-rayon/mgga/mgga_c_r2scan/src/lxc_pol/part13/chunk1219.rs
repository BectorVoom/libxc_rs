//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1219/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1219(t32212: f64, t481: f64, t14160: f64, t40630: f64, t11550: f64, t792: f64, t3262: f64, t3276: f64, t10648: f64, t10971: f64, t11564: f64, t10610: f64, t3263: f64) -> (f64, f64, f64, f64) {
    let t40631 = t32212 * t481;
    let t40634 = 3.0_f64 * t40630 * t14160 * t40631;
    let t40635 = t11550 * t792;
    let t40638 = 15.0_f64 / 8.0_f64 * t3262 * t3276 * t40635;
    let t40642 = t10648 * t10971 * t11564;
    let t40644 = t11550 * t481;
    let t40647 = 3.0_f64 * t10610 * t3263 * t40644;
    (t40634, t40638, t40642, t40647)
}
