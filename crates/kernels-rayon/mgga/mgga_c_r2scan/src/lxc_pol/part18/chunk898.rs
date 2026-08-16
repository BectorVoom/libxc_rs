//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 898/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk898(t298: f64, t9612: f64, t2916: f64, t6635: f64, t810: f64, t1000: f64, t35: f64, t1216: f64, t1256: f64, t2920: f64, t308: f64, t2369: f64, t2373: f64, t2901: f64, t2905: f64, t2911: f64, t295: f64, t305: f64, t6648: f64, t803: f64, t811: f64, t8319: f64, t8340: f64, t9598: f64, t9602: f64, t9608: f64, t997: f64) -> (f64, f64, f64, f64, f64) {
    let t9613 = t298 * t9612;
    let t9622 = t6635 * t2916;
    let t9623 = t9622 * t810;
    let t9626 = t1000 * t35;
    let t9627 = t9626 * t1216;
    let t9630 = t1256 * t2920;
    let t9631 = t9630 * t810;
    let t9634 = -t9612;
    let t9635 = t308 * t9634;
    let t9638 = -50.0_f64 / 27.0_f64 * t803 * t2901 - 10.0_f64 / 27.0_f64 * t295 * t9598 + 20.0_f64 / 9.0_f64 * t8319 * t9602 - 25.0_f64 / 9.0_f64 * t803 * t2905 + 10.0_f64 / 9.0_f64 * t295 * t9608 + 5.0_f64 / 3.0_f64 * t295 * t9613 + 200.0_f64 / 27.0_f64 * t2911 * t811 - 100.0_f64 / 27.0_f64 * t997 * t2369 + 50.0_f64 / 9.0_f64 * t997 * t2373 - 10.0_f64 / 27.0_f64 * t305 * t9623 - 20.0_f64 / 9.0_f64 * t8340 * t9627 + 10.0_f64 / 9.0_f64 * t305 * t9631 + 5.0_f64 / 3.0_f64 * t305 * t9635 + t6648;
    (t9613, t9623, t9631, t9635, t9638)
}
