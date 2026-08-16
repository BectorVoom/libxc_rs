//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1290/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1290(t22716: f64, t8622: f64, t6897: f64, t80645: f64, t8621: f64, t22704: f64, t31559: f64, t81326: f64, t2085: f64, t212: f64, t22642: f64, t6890: f64) -> (f64, f64, f64, f64) {
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1_f64 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    let t115318 = t22704 * t81326 * t31559;
    let t115330 = t22642 * t212 * t2085 * t6890;
    (t115306, t115308, t115318, t115330)
}
