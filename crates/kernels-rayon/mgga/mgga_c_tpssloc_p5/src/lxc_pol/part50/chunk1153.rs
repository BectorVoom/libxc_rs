//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1153/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1153(t113981: f64, t1369: f64, t31176: f64, t22804: f64, t31156: f64, t31169: f64, t3777: f64, t1336: f64, t1338: f64, t241: f64, t835: f64, t31172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113982 = 0.6728792682356731809e-4_f64 * t113981;
    let t113987 = t31176 * t1369;
    let t114000 = t22804 * t31156;
    let t114002 = t3777 * t31169;
    let t114011 = t1336 * t1338 * t835 * t241;
    let t114012 = t114011 * t31172;
    (t113982, t113987, t114000, t114002, t114011, t114012)
}
