//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 876/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk876(t506: f64, t8629: f64, t529: f64, t552: f64, t551: f64, t1567: f64, t3055: f64, t1569: f64, t2115: f64, t1604: f64, t2214: f64, t3197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9301 = t506 * t8629;
    let t9302 = t529 * t9301;
    let t9311 = t552 * t8629;
    let t9312 = t551 * t9311;
    let t9317 = t1567 * t3055;
    let t9318 = t9317 * t1569;
    let t9319 = t2115 * t9318;
    let t9320 = t1604 * t9319;
    let t9322 = t2214 * t3197;
    (t9302, t9311, t9312, t9317, t9318, t9319, t9320, t9322)
}
