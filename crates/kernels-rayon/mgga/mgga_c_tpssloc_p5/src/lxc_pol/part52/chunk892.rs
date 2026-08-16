//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 892/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk892(t533: f64, t8492: f64, t3701: f64, t1983: f64, t1873: f64, t7010: f64, t3941: f64, t8319: f64, t1401: f64, t8326: f64, t131: f64, t8306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8493 = t533 * t8492;
    let t8494 = t8493 * t3701;
    let t8495 = t1983 * t8494;
    let t8503 = t7010 * t1873;
    let t8506 = 27.0_f64 * t3941 * t8319;
    let t8508 = 0.135e2_f64 * t1401 * t8326;
    let t8513 = t131 * t8306;
    (t8493, t8494, t8495, t8503, t8506, t8508, t8513)
}
