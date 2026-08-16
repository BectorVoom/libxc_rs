//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 932/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk932(t28800: f64, t7303: f64, t7302: f64, t2579: f64, t9078: f64, t1948: f64, t28294: f64, t5322: f64, t5321: f64, t28749: f64, t7316: f64, t7315: f64) -> (f64, f64, f64, f64) {
    let t29541 = t7303 * t28800;
    let t29542 = t7302 * t29541;
    let t29544 = t9078 * t2579;
    let t29545 = t1948 * t29544;
    let t29547 = t5322 * t28294;
    let t29548 = t5321 * t29547;
    let t29550 = t7316 * t28749;
    let t29551 = t7315 * t29550;
    (t29542, t29545, t29548, t29551)
}
