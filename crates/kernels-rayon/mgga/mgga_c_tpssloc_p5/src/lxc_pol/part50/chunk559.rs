//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 559/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk559(t1527: f64, t865: f64, t2718: f64, t2627: f64, t68: f64, t226: f64, t1509: f64, t252: f64, t4182: f64, t1510: f64, t2732: f64, t4234: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4272 = t1527 * t865;
    let t4273 = t2718 * t4272;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4282 = t252 * t1509;
    let t4283 = t4282 * t4182;
    let t4286 = t2732 * t1510;
    let t4288 = t860 * t4234;
    (t4272, t4273, t4281, t4282, t4283, t4286, t4288)
}
