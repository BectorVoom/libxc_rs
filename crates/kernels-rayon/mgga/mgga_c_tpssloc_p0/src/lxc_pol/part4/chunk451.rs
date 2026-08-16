//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 451/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk451(t1557: f64, t893: f64, t1541: f64, t917: f64, t1548: f64, t1551: f64, t1554: f64, t926: f64, t929: f64, t932: f64) -> (f64, f64, f64, f64) {
    let t1559 = 1.0_f64 * t893 * t1557;
    let t1561 = -t917 - 0.17123333333333333333e-1_f64 * t1541;
    let t1568 = 0.3529725e1_f64 * t1548 - t926 - 0.516475e0_f64 * t1541 + 0.6311625e0_f64 * t1551 - t929 - 0.104195e0_f64 * t1554;
    let t1569 = t1568 * t932;
    (t1559, t1561, t1568, t1569)
}
