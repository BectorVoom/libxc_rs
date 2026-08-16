//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2265/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2265(t1437: f64, t6509: f64, t1863: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t26013: f64, t83722: f64, t83741: f64, t83778: f64, t90072: f64, t90076: f64, t90080: f64, t90087: f64) -> f64 {
    let t90090 = t6509 * t1437;
    let t90091 = t1863 * t90090;
    let t90094 = t1864 * t4021;
    let t90095 = t1863 * t90094;
    let t90098 = t9231 * t1410;
    let t90101 = t2240 * t3961;
    let t90104 = t2240 * t3967;
    let t90107 = -10.0_f64 * t83741 * t26009 - 10.0_f64 * t22544 * t90072 - 10.0_f64 * t22544 * t90076 - 5.0_f64 * t22544 * t90080 - 10.0_f64 / 3.0_f64 * t83722 * t26013 - 5.0_f64 / 3.0_f64 * t83778 * t26013 - 10.0_f64 / 3.0_f64 * t22549 * t90087 - 10.0_f64 / 3.0_f64 * t22549 * t90091 - 10.0_f64 / 3.0_f64 * t22549 * t90095 - 10.0_f64 / 3.0_f64 * t90098 * t22551 - 10.0_f64 / 3.0_f64 * t90101 * t22551 - 10.0_f64 / 3.0_f64 * t90104 * t22551;
    t90107
}
