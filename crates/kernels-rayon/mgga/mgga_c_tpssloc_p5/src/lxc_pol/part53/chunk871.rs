//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 871/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk871(t31405: f64, t6562: f64, t6571: f64, t7106: f64, t6553: f64, t1880: f64, t6547: f64, t8548: f64, t1377: f64, t2091: f64, t1307: f64, t22635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31406 = t6562 * t31405;
    let t31419 = t6571 * t7106;
    let t31420 = t6553 * t31419;
    let t31421 = t1880 * t31420;
    let t31425 = t6547 * t8548;
    let t31549 = t1377 * t2091;
    let t31550 = t31549 * t1307;
    let t31551 = t22635 * t31550;
    (t31406, t31419, t31420, t31421, t31425, t31549, t31550, t31551)
}
