//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 762/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk762(t2229: f64, t61: f64, t119: f64, t212: f64, t252: f64, t828: f64, t1929: f64, t343: f64, t984: f64, t3034: f64, t334: f64, rho0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6597 = 1.0_f64 / t61 / t2229;
    let t6600 = t119 * t212;
    let t6647 = t252 * t828;
    let t6720 = t1929 * rho0;
    let t6733 = t984 * t343;
    let t6739 = 1.0_f64 / t3034 / t334;
    (t6597, t6600, t6647, t6720, t6733, t6739)
}
