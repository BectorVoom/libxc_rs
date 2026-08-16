//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 386/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk386(t1808: f64, t1809: f64, t2399: f64, t2477: f64, t2488: f64, t2505: f64, t604: f64, t674: f64, t702: f64) -> f64 {
    let t2507 = -t1808 - 0.23426533963880895498e-2_f64 * t1809 * t2477 - 0.46853067927761790996e-2_f64 * t674 * t2488 - t2399 * t702 - t604 * t2505;
    t2507
}
