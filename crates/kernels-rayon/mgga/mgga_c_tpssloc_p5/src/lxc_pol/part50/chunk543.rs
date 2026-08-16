//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 543/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk543(t1489: f64, t2563: f64, t131: f64, t2570: f64, t205: f64, t1484: f64, t213: f64, t221: f64, t776: f64, t118: f64, t794: f64, t2576: f64) -> (f64, f64, f64, f64) {
    let t4124 = t2563 * t1489;
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4128 = t213 * t1484;
    let t4130 = t221 * t4128 * t776;
    let t4134 = t118 * t794 * t1484;
    let t4135 = t2576 * t4134;
    (t4124, t4127, t4130, t4135)
}
