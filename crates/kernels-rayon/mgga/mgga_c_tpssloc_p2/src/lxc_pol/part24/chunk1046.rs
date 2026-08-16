//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1046/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1046(t11115: f64, t11967: f64, t510: f64, t9416: f64, t3696: f64, t588: f64, t592: f64, t1285: f64, t2223: f64, t1287: f64, t1291: f64, t9874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11968 = t11115 + t11967;
    let t11972 = t510 * t9416;
    let t11975 = t588 * t3696;
    let t11976 = 12.0_f64 * t11975;
    let t11977 = t592 * t3696;
    let t11978 = 12.0_f64 * t11977;
    let t11979 = t2223 * t1285;
    let t11980 = 96.0_f64 * t11979;
    let t11981 = t2223 * t1287;
    let t11982 = 96.0_f64 * t11981;
    let t11984 = 0.56968947174242584612e-3_f64 * t1291 * t9874;
    (t11968, t11972, t11976, t11978, t11980, t11982, t11984)
}
