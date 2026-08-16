//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 241/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk241(t1077: f64, t1078: f64, t47: f64, t841: f64, t846: f64, t848: f64, t2: f64, t240: f64, t1072: f64, t157: f64, t251: f64, t5: f64, t56: f64, t812: f64, t833: f64, t836: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1079 = t1077 * t1078;
    let t1085 = t47 * t841;
    let t1086 = t846 * t848;
    let t1091 = t240 * t2;
    let t1094 = t240 * t47;
    let t1097 = t812 + t833 + t240 * (0.53236443333333333332e-3_f64 * t5 * t157 * t251 + 1.0_f64 * t1072 * t1079 - t812 - t833 + 0.18311555036753159941e-3_f64 * t5 * t157 * t56 + 0.58482233974552040708e0_f64 * t1085 * t1086) - 0.18311555036753159941e-3_f64 * t1091 * t836 - 0.58482233974552040708e0_f64 * t1094 * t849;
    (t1079, t1085, t1086, t1091, t1094, t1097)
}
