//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2197/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197(t22574: f64, t74060: f64, t8643: f64, t1388: f64, t28830: f64, t26162: f64, t1983: f64, t28238: f64, t6999: f64, t75214: f64, t12461: f64, t7752: f64) -> (f64, f64, f64, f64, f64) {
    let t97910 = 6.0_f64 * t22574 * t8643 * t74060;
    let t97911 = t28830 * t1388;
    let t97914 = 12.0_f64 * t22574 * t26162 * t97911;
    let t97916 = t1983 * t28238 * t6999;
    let t97919 = 3.0_f64 * t22574 * t8643 * t75214;
    let t97920 = t7752 * t12461;
    (t97910, t97914, t97916, t97919, t97920)
}
