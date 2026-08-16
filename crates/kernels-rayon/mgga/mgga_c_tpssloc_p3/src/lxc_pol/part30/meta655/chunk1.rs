//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2072/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2072(t1437: f64, t6509: f64, t1863: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t12571: f64, t608: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90090 = t6509 * t1437;
    let t90091 = t1863 * t90090;
    let t90094 = t1864 * t4021;
    let t90095 = t1863 * t90094;
    let t90098 = t9231 * t1410;
    let t90101 = t2240 * t3961;
    let t90104 = t2240 * t3967;
    let t90114 = t12571 * t608;
    (t90091, t90095, t90098, t90101, t90104, t90114)
}
