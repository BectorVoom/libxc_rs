//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1290/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1290(t10143: f64, t8565: f64, t31650: f64, t6883: f64, t31608: f64, t1377: f64, t7213: f64, t22716: f64, t8622: f64, t6897: f64, t80645: f64, t8621: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115027 = t8565 * t10143;
    let t115292 = t6883 * t31650;
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1_f64 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    (t115027, t115292, t115294, t115296, t115306, t115308)
}
