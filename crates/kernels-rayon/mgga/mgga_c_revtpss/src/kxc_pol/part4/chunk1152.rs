//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1152/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1152(t14151: f64, t14200: f64, t14237: f64, t14266: f64, t1427: f64, t1904: f64, t3899: f64, t689: f64, t10151: f64, t10154: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t1424: f64, t4132: f64, t5715: f64, t9695: f64) -> f64 {
    let t14268 = t14151 + t14200 + t14237 + t14266;
    let t14269 = t1427 * t14268;
    let t14274 = t3899 * t1904;
    let t14276 = 0.10975748638225852664e-1_f64 * t689 * t14274;
    let t14279 = 0.13009920719177044025e-1_f64 * t14091 - 0.2601984143835408805e-1_f64 * t9695 + t14096 + 0.73171657588172351096e-2_f64 * t14097 - t14102 - 0.11565819519348392139e-2_f64 * t14105 - t14108 + 0.39029762157531132075e-1_f64 * t14111 - 0.65854491829355115987e0_f64 * t1424 * t14269 - 0.65854491829355115987e0_f64 * t5715 * t4132 + t14276 - 0.10975748638225852664e-1_f64 * t10151 + 0.10975748638225852664e-1_f64 * t10154;
    t14279
}
