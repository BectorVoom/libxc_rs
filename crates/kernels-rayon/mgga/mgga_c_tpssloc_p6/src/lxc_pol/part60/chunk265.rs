//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 265/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk265(t1573: f64, t324: f64, t1541: f64, t1548: f64, t1551: f64, t1554: f64, t945: f64, t948: f64, t951: f64, t1545: f64, t1559: f64, t1561: f64, t1569: f64, t300: f64, t311: f64, t924: f64, t943: f64) -> (f64, f64, f64, f64) {
    let t1574 = t1573 * t324;
    let t1580 = 0.258925e1_f64 * t1548 - t945 - 0.301925e0_f64 * t1541 + 0.16504875e0_f64 * t1551 - t948 - 0.82785e-1_f64 * t1554;
    let t1581 = t1580 * t951;
    let t1585 = t300 * (-0.310907e-1_f64 * t1561 * t311 + 1.0_f64 * t924 * t1569 + t1545 - t1559 - 0.19751673498613801407e-1_f64 * t1574 + 0.5848223622634646207e0_f64 * t943 * t1581);
    (t1574, t1580, t1581, t1585)
}
