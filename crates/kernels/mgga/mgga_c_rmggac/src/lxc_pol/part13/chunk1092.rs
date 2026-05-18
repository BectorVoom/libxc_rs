//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1092/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1092<F: Float>(t41654: F, t41656: F, t41667: F, t1364: F, t1668: F, t36521: F, t36533: F, t36535: F, t37976: F, t41663: F, t41669: F, t41672: F, t41675: F, t41690: F, t41694: F, t41696: F, t5184: F, t699: F, t8188: F, t931: F, t9639: F) -> F {
    let t43783 = F::new(0.11918087970123395032e-3) * t41654;
    let t43784 = F::new(0.36366215538993788974e-1) * t41656;
    let t43792 = F::new(0.86737941314158990616e-4) * t41667;
    let t43802 = -F::new(0.16552899958504715322e-3) * t36521 + t43783 - t43784 + t37976 - F::new(0.2363e1) * t931 * t9639 - F::new(0.4726e1) * t1668 * t8188 + F::new(0.35754263910370185094e-3) * t36533 + F::new(0.11918087970123395032e-3) * t36535 + F::new(0.85129199786595678799e-5) * t41663 + t43792 - F::new(0.11974241701863808564e0) * t41669 + F::new(0.35922725105591425692e0) * t41672 - F::new(0.71845450211182851384e0) * t41675 - F::new(0.23948483403727617128e0) * t1364 * t699 * t5184 + F::new(0.5107751987195740728e-4) * t41690 - F::new(0.5107751987195740728e-4) * t41694 + F::new(0.212822999466489197e-4) * t41696;
    t43802
}
