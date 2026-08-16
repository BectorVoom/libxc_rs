//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1245/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1245(t1873: f64, t90400: f64, t120112: f64, t112594: f64, t119815: f64, t119820: f64, t119996: f64, t120140: f64, t120141: f64, t120143: f64, t120146: f64, t120149: f64, t120151: f64, t120153: f64, t1458: f64, t31224: f64, t4072: f64, t671: f64) -> f64 {
    let t120163 = t90400 * t1873;
    let t120165 = 2.0_f64 * t120112;
    let t120166 = 2.0_f64 * t112594 * t1458 + 2.0_f64 * t119815 * t671 + 2.0_f64 * t119820 * t1458 + 2.0_f64 * t31224 * t4072 + t119996 + t120140 + 4.0_f64 * t120141 + 4.0_f64 * t120143 + 4.0_f64 * t120146 + 4.0_f64 * t120149 + 4.0_f64 * t120151 + 4.0_f64 * t120153 + 4.0_f64 * t120163 + t120165;
    t120166
}
