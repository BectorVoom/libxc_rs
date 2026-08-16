//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 900/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk900(t1921: f64, t23587: f64, t3034: f64, t38: f64, t131: f64, t350: f64, t3030: f64, t344: f64, t225: f64, t6733: f64, t1949: f64, t2966: f64) -> (f64, f64, f64, f64, f64) {
    let t23588 = t1921 * t23587;
    let t23598 = 1.0_f64 / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    let t23601 = t23600 * t350;
    let t23602 = t344 * t3030;
    let t23613 = t6733 * t225;
    let t23617 = t2966 * t1949;
    (t23588, t23601, t23602, t23613, t23617)
}
