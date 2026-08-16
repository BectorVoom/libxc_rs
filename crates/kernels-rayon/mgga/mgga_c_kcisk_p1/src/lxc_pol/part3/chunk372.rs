//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 372/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk372(t1689: f64, t1808: f64, t1809: f64, t1810: f64, t1825: f64, t1860: f64, t604: f64, t674: f64, t702: f64) -> f64 {
    let t1862 = -t1808 - 0.23426533963880895498e-2_f64 * t1809 * t1810 - 0.46853067927761790996e-2_f64 * t674 * t1825 - t1689 * t702 - t604 * t1860;
    t1862
}
