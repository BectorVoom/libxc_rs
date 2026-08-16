//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 242/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk242<F: Float>(t1077: F, t1078: F, t47: F, t841: F, t846: F, t848: F, t2: F, t240: F, t1072: F, t157: F, t251: F, t5: F, t56: F, t812: F, t833: F, t836: F, t849: F) -> (F, F, F, F, F, F) {
    let t1079 = t1077 * t1078;
    let t1085 = t47 * t841;
    let t1086 = t846 * t848;
    let t1091 = t240 * t2;
    let t1094 = t240 * t47;
    let t1097 = t812 + t833 + t240 * (F::cast_from(0.53236443333333333332e-3_f64) * t5 * t157 * t251 + F::cast_from(1.0_f64) * t1072 * t1079 - t812 - t833 + F::cast_from(0.18311555036753159941e-3_f64) * t5 * t157 * t56 + F::cast_from(0.58482233974552040708e0_f64) * t1085 * t1086) - F::cast_from(0.18311555036753159941e-3_f64) * t1091 * t836 - F::cast_from(0.58482233974552040708e0_f64) * t1094 * t849;
    (t1079, t1085, t1086, t1091, t1094, t1097)
}
