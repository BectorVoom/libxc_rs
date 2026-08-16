//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 720/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk720(t10842: f64, t11188: f64, t1791: f64, t1691: f64, t604: f64, t1790: f64, t4824: f64, t4825: f64, t667: f64, t1692: f64, t4794: f64, t10471: f64, t140: f64, t673: f64) -> (f64, f64, f64, f64, f64) {
    let t11189 = t10842 + t11188;
    let t11190 = t11189 * t1791;
    let t11195 = t1691 * t1691;
    let t11196 = 1.0_f64 / t11195;
    let t11197 = t604 * t11196;
    let t11198 = t4824 * t1790;
    let t11200 = 1.0_f64 / t4825 / t667;
    let t11201 = t11198 * t11200;
    let t11204 = t4794 * t1692;
    let t11208 = t140 * t10471 * t673;
    (t11190, t11197, t11201, t11204, t11208)
}
