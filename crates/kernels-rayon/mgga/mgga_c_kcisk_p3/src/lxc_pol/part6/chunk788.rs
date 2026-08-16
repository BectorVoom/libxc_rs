//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 788/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk788(t2250: f64, t979: f64, t1390: f64, t2209: f64, t3532: f64, t2242: f64, t306: f64, t140: f64, t2253: f64, t430: f64, t2257: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21152 = t979 * t2250;
    let t21230 = t2209 * t1390;
    let t21239 = t2209 * t3532;
    let t21252 = t2242 * t306;
    let t21256 = t140 * t430 * t2253;
    let t21314 = t2257 * t3783;
    (t21152, t21230, t21239, t21252, t21256, t21314)
}
