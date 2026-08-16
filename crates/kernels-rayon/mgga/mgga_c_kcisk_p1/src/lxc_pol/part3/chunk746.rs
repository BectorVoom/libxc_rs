//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 746/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk746(t5814: f64, t79: f64, t435: f64, t690: f64, t41: f64, t5821: f64, t698: f64, t445: f64, t1836: f64, t3114: f64, t1843: f64, t3119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11525 = t5814 * t79;
    let t11528 = 0.77488888888888888888e-2_f64 * t435 * t11525 * t690;
    let t11529 = t5821 * t41;
    let t11530 = t11529 * t698;
    let t11532 = 0.72818958333333333333e-4_f64 * t445 * t11530;
    let t11533 = t3114 * t1836;
    let t11535 = t3119 * t1843;
    (t11525, t11528, t11529, t11532, t11533, t11535)
}
