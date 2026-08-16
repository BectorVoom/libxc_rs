//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 987/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk987(t1336: f64, t16397: f64, t5252: f64, t225: f64, t5319: f64, t5217: f64, t1390: f64, t5356: f64, t1395: f64, t1858: f64, t5381: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16398 = t1336 * t16397;
    let t16400 = 7.0_f64 / 1152.0_f64 * t16398 * t5252;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16513 = 2.0_f64 * t1395 * t1858;
    let t16515 = 2.0_f64 * t576 * t5381;
    (t16400, t16439, t16460, t16497, t16513, t16515)
}
