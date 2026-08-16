//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2076/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2076(t1032: f64, t10375: f64, t370: f64, t374: f64, t376: f64, t9697: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64) -> (f64, f64, f64, f64, f64) {
    let t43248 = t1032 * t10375;
    let t43253 = 7.0_f64 / 31104.0_f64 * t370 * t374 * t9697 * t376;
    let t43288 = 1.0_f64 / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    (t43248, t43253, t43288, t43291, t43292)
}
