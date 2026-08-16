//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 592/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk592(t1409: f64, t2770: f64, t2775: f64, t1543: f64, t892: f64, t1547: f64, t2798: f64, t2815: f64, t1553: f64, t699: f64, t1561: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4337 = t2770 * t1409;
    let t4342 = t2775 * t1409;
    let t4354 = t1543 * t892;
    let t4362 = t2798 * t1547;
    let t4378 = t2815 * t1547;
    let t4384 = t699 * t1553;
    let t4411 = t1561 * t923;
    (t4337, t4342, t4354, t4362, t4378, t4384, t4411)
}
