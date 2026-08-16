//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 938/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk938(t226: f64, t235: f64, t2690: f64, t8344: f64, t23139: f64, t8339: f64, t79: f64, t8306: f64, t22779: f64, t31162: f64, t22817: f64, t794: f64, t8462: f64) -> (f64, f64, f64, f64, f64) {
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112855 = t23139 * t8339;
    let t113875 = t8306 * t79;
    let t113966 = t22779 * t31162;
    let t113981 = t22817 * t794 * t8462;
    (t112850, t112855, t113875, t113966, t113981)
}
