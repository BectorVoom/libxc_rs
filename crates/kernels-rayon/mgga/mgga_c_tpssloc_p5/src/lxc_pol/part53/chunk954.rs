//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 954/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk954(t3886: f64, t7213: f64, t22724: f64, t31569: f64, t31589: f64, t6897: f64, t794: f64, t22573: f64, t8606: f64, t32281: f64, t580: f64, t1404: f64, t8811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115614 = t3886 * t7213;
    let t115629 = t22724 * t31569;
    let t115658 = t6897 * t794 * t31589;
    let t115925 = t8606 * t22573;
    let t116385 = t32281 * t580;
    let t116387 = t8811 * t1404;
    (t115614, t115629, t115658, t115925, t116385, t116387)
}
