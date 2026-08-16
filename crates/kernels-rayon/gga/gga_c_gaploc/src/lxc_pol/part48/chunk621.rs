//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 621/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk621(t11218: f64, t493: f64, t492: f64, t169: f64, t172: f64, t452: f64, t10194: f64, t10228: f64, t10239: f64, t10254: f64, t10260: f64, t10263: f64, t105: f64, t3532: f64, t3537: f64, t3542: f64, t419: f64) -> (f64, f64, f64) {
    let t11219 = t493 * t11218;
    let t11220 = t492 * t11219;
    let t11232 = t11218 * t169 * t172;
    let t11233 = t452 * t11232;
    let t11240 = -0.28455006635676149599e-1_f64 * t105 * t11220 - 0.85365019907028448797e-1_f64 * t419 * t3537 + 0.56910013271352299198e-1_f64 * t419 * t3542 - 0.47425011059460249332e-2_f64 * t10194 + 0.47425011059460249332e-2_f64 * t10228 + 0.28455006635676149599e-1_f64 * t419 * t3532 + 0.28455006635676149599e-1_f64 * t105 * t11233 - 0.63233348079280332443e-2_f64 * t10239 - 0.47425011059460249332e-2_f64 * t10254 + 0.47425011059460249332e-2_f64 * t10260 + 0.47425011059460249332e-2_f64 * t10263;
    (t11219, t11232, t11240)
}
