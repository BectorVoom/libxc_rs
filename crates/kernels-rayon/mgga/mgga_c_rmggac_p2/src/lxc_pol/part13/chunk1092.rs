//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1092/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1092(t41654: f64, t41656: f64, t41667: f64, t1364: f64, t1668: f64, t36521: f64, t36533: f64, t36535: f64, t37976: f64, t41663: f64, t41669: f64, t41672: f64, t41675: f64, t41690: f64, t41694: f64, t41696: f64, t5184: f64, t699: f64, t8188: f64, t931: f64, t9639: f64) -> f64 {
    let t43783 = 0.11918087970123395032e-3_f64 * t41654;
    let t43784 = 0.36366215538993788974e-1_f64 * t41656;
    let t43792 = 0.86737941314158990616e-4_f64 * t41667;
    let t43802 = -0.16552899958504715322e-3_f64 * t36521 + t43783 - t43784 + t37976 - 0.2363e1_f64 * t931 * t9639 - 0.4726e1_f64 * t1668 * t8188 + 0.35754263910370185094e-3_f64 * t36533 + 0.11918087970123395032e-3_f64 * t36535 + 0.85129199786595678799e-5_f64 * t41663 + t43792 - 0.11974241701863808564e0_f64 * t41669 + 0.35922725105591425692e0_f64 * t41672 - 0.71845450211182851384e0_f64 * t41675 - 0.23948483403727617128e0_f64 * t1364 * t699 * t5184 + 0.5107751987195740728e-4_f64 * t41690 - 0.5107751987195740728e-4_f64 * t41694 + 0.212822999466489197e-4_f64 * t41696;
    t43802
}
