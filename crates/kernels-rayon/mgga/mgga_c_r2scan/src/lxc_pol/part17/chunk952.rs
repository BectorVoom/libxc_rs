//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 952/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk952(t11031: f64, t11057: f64, t3506: f64, t833: f64, t1120: f64, t1299: f64, t1338: f64, t3552: f64, t1142: f64, t6755: f64, t1348: f64, t6767: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11206 = 22.0_f64 / 9.0_f64 * t11031;
    let t11215 = 22.0_f64 / 9.0_f64 * t11057;
    let t11220 = t3506 * t833;
    let t11223 = t1120 * t1299;
    let t11302 = t1338 * t3552;
    let t11305 = t6755 * t1142;
    let t11314 = t1348 * t3552;
    let t11319 = t6767 * t1142;
    (t11206, t11215, t11220, t11223, t11302, t11305, t11314, t11319)
}
