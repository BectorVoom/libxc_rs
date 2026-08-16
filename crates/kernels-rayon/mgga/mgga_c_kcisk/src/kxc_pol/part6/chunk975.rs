//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 975/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk975(t30184: f64, t5625: f64, t3796: f64, t3482: f64, t2152: f64, t7706: f64, t14255: f64, t3484: f64, t5633: f64, t5606: f64, t8082: f64, t1411: f64) -> (f64, f64, f64, f64) {
    let t30185 = t5625 * t30184;
    let t30186 = t3796 * t30185;
    let t30187 = t3482 * t30186;
    let t30189 = t7706 * t2152;
    let t30190 = t14255 * t30189;
    let t30191 = t3484 * t30190;
    let t30192 = t5633 * t30191;
    let t30194 = t5606 * t8082;
    let t30195 = t1411 * t30194;
    (t30187, t30189, t30192, t30195)
}
