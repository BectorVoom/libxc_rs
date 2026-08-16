//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1161/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161(t12379: f64, t3799: f64, t12384: f64, t3777: f64, t3795: f64, t3792: f64, t39937: f64, t12282: f64, t3809: f64, t12328: f64, t1333: f64, t12012: f64, t12351: f64, t12368: f64, t1307: f64, t1343: f64, t1354: f64, t1363: f64, t3719: f64, t3734: f64, t3790: f64, t3803: f64, t3851: f64, t3870: f64, t40114: f64, t40116: f64, t40119: f64, t40124: f64, t40126: f64, t5248: f64, t820: f64) -> (f64, f64) {
    let t40128 = t3799 * t12379;
    let t40130 = t3777 * t12384;
    let t40131 = t40130 * t3795;
    let t40133 = t39937 * t3792;
    let t40138 = t3777 * t12282;
    let t40139 = t40138 * t3809;
    let t40145 = t1333 * t12328;
    let t40147 = -15.0_f64 / 64.0_f64 * t1363 * t12351 * t820 * t3734 * t3719 + 5.0_f64 / 192.0_f64 * t1363 * t3870 * t820 * t1307 * t12012 + 7.0_f64 / 384.0_f64 * t40114 - 35.0_f64 / 96.0_f64 * t40116 - t40119 * t1354 / 768.0_f64 + 595.0_f64 / 2592.0_f64 * t40124 - 119.0_f64 / 2304.0_f64 * t40126 + 7.0_f64 / 1152.0_f64 * t40128 - 7.0_f64 / 192.0_f64 * t40131 + t3790 * t1343 * t820 * t40133 / 512.0_f64 - 7.0_f64 / 48.0_f64 * t40139 - t3803 * t5248 * t12368 * t3851 / 512.0_f64 - 595.0_f64 / 2592.0_f64 * t40145;
    (t40133, t40147)
}
