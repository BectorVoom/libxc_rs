//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 331/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk331(t1589: f64, t959: f64, t1409: f64, t978: f64, t977: f64, t1554: f64, t906: f64, t340: f64, t343: f64, t974: f64, t971: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1591 = 0.5848223622634646207e0_f64 * t959 * t1589;
    let t1592 = t978 * t1409;
    let t1593 = t977 * t1592;
    let t1597 = t906 / 6.0_f64 + t1554 / 6.0_f64;
    let t1598 = t340 * t1597;
    let t1599 = t1598 * t343;
    let t1600 = t974 * t1599;
    let t1603 = t971 + 0.27777777777777777777e-3_f64 * t973 * t1593 - 0.83333333333333333332e-3_f64 * t973 * t1600;
    (t1591, t1592, t1593, t1597, t1599, t1600, t1603)
}
