//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 411/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk411(t1305: f64, t1328: f64, t1309: f64, t1320: f64, t1325: f64, t1332: f64) -> (f64, f64, f64) {
    let t1566 = 0.516475e0_f64 * t1305;
    let t1569 = 0.104195e0_f64 * t1328;
    let t1571 = 0.3529725e1_f64 * t1320 - t1566 - 0.516475e0_f64 * t1309 + 0.6311625e0_f64 * t1325 - t1569 - 0.104195e0_f64 * t1332;
    (t1566, t1569, t1571)
}
