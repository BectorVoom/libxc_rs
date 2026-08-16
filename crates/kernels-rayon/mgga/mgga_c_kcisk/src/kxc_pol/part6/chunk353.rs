//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 353/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk353(t1348: f64, t1349: f64, t2110: f64, t2181: f64, t2192: f64, t2209: f64, t338: f64, t417: f64, t451: f64) -> f64 {
    let t2211 = -t1348 - 0.23426533963880895498e-2_f64 * t1349 * t2181 - 0.46853067927761790996e-2_f64 * t417 * t2192 - t2110 * t451 - t338 * t2209;
    t2211
}
