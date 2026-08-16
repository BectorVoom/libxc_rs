//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 524/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk524(t1474: f64, t4265: f64, t140: f64, t1477: f64, t299: f64, t3529: f64, t41: f64, t3532: f64, t451: f64, t3278: f64, t1402: f64, t442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4266 = t4265 * t1474;
    let t4269 = t140 * t299 * t1477;
    let t4271 = t41 * t3529;
    let t4272 = t451 * t3532;
    let t4274 = t4271 * t4272 * t3278;
    let t4277 = t1402 * t442;
    (t4266, t4269, t4271, t4272, t4274, t4277)
}
