//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 679/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk679(t10933: f64, t606: f64, t11032: f64, t1848: f64, t641: f64, t916: f64, t5014: f64, t5030: f64, t1691: f64, t604: f64, t4825: f64, t667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11040 = 28.0_f64 / 27.0_f64 * t10933;
    let t11056 = 1.0_f64/pow_3_2(t606);
    let t11091 = 0.93932222222222222223e0_f64 * t10933;
    let t11092 = 0.73586666666666666667e0_f64 * t11032;
    let t11105 = 0.55403703703703703703e-1_f64 * t10933;
    let t11153 = 1.0_f64 / t641 / t916 / t1848;
    let t11179 = t5014 * t5030;
    let t11195 = t1691 * t1691;
    let t11196 = 1.0_f64 / t11195;
    let t11197 = t604 * t11196;
    let t11200 = 1.0_f64 / t4825 / t667;
    (t11040, t11056, t11091, t11092, t11105, t11153, t11179, t11197, t11200)
}
