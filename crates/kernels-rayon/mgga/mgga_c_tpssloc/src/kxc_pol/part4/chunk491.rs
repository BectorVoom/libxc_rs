//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 491/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk491(t25: f64, t28: f64, t17: f64, t1788: f64, t1787: f64, t182: f64, t1298: f64, t1408: f64, t1302: f64, t1649: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1789 = t17 * t1788;
    let t1791 = 0.19751673498613801407e-1_f64 * t1787 * t182;
    let t1794 = piecewise3(t26, 0.0_f64, 2.0_f64 / 3.0_f64 * t1298 * t1408);
    let t1797 = piecewise3(t29, 0.0_f64, 2.0_f64 / 3.0_f64 * t1302 * t1649);
    let t1799 = t1794 / 2.0_f64 + t1797 / 2.0_f64;
    (t1789, t1791, t1799)
}
