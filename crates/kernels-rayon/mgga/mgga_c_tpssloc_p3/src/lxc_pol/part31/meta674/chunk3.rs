//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2039/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039(t29430: f64, t576: f64, t1858: f64, t7945: f64, t29395: f64, t580: f64, t2098: f64, t6483: f64, t101021: f64, t103073: f64, t103088: f64, t1396: f64, t1398: f64, t1852: f64, t27286: f64, t3: f64, t6471: f64, t7240: f64, t94113: f64, t94118: f64, t94120: f64, t94122: f64) -> f64 {
    let t103091 = t576 * t29430;
    let t103092 = t7945 * t1858;
    let t103098 = t29395 * t580;
    let t103099 = t2098 * t6483;
    let t103102 = t1398 * (t101021 + t103088) + t103091 + 2.0_f64 * t103092 + t6471 * t7240 + t1396 * t29430 + 2.0_f64 * t1852 * t27286 + t94113 + t103098 + t103099 + t94118 + t94120 + t94122 + t3 * t103073 * t580;
    t103102
}
