//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3126/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126(t15376: f64, t15420: f64, t15419: f64, t18211: f64, t3447: f64, t11575: f64, t11579: f64, t11584: f64, t15268: f64, t15321: f64, t18409: f64, t18416: f64, t18420: f64, t4908: f64, t51975: f64, t52013: f64, t63298: f64, t63302: f64) -> f64 {
    let t64667 = t15376 * t15420;
    let t64686 = t3447 * t15419 * t18211;
    let t64694 = -0.2962962962962962963e-2_f64 * t15376 * t15321 + 0.27777777777777777777e-3_f64 * t3447 * t11575 * t18409 - 0.13168724279835390946e-2_f64 * t64667 - 0.55555555555555555554e-3_f64 * t3447 * t4908 * t63298 - 0.16666666666666666666e-2_f64 * t3447 * t4908 * t63302 + 0.27777777777777777777e-3_f64 * t3447 * t18416 * t11579 + 0.55555555555555555554e-3_f64 * t3447 * t18416 * t11584 - 0.33333333333333333332e-2_f64 * t3447 * t51975 * t15268 - 0.55555555555555555554e-3_f64 * t52013 + 0.14814814814814814814e-2_f64 * t64686 + 0.27777777777777777777e-3_f64 * t3447 * t18420 * t11579 + 0.55555555555555555554e-3_f64 * t3447 * t18420 * t11584;
    t64694
}
