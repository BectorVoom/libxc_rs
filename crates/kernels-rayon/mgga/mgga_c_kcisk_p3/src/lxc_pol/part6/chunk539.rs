//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 539/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk539(t707: f64, t725: f64, t2551: f64, t4265: f64, t4594: f64, t702: f64, t1797: f64, t140: f64, t2554: f64, t299: f64, t2505: f64, t695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7360 = t725 * t707;
    let t7368 = t4265 * t2551;
    let t7370 = t4594 * t702;
    let t7378 = t1797 * t702;
    let t7387 = t140 * t299 * t2554;
    let t7389 = t2505 * t695;
    (t7360, t7368, t7370, t7378, t7387, t7389)
}
