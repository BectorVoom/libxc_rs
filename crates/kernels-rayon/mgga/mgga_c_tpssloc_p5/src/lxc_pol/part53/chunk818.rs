//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 818/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk818(t26396: f64, t6637: f64, t6888: f64, t6914: f64, t7737: f64, t1351: f64, t1834: f64, t550: f64, t6976: f64, t1992: f64, t3807: f64, t5335: f64) -> (f64, f64, f64, f64) {
    let t26397 = t6637 * t26396;
    let t26398 = t6888 * t26397;
    let t26406 = t6914 * t7737;
    let t26409 = t1834 * t1351;
    let t26410 = t26409 * t550;
    let t26411 = t6976 * t26410;
    let t26412 = t1992 * t26411;
    let t26414 = t5335 * t3807;
    (t26398, t26406, t26412, t26414)
}
