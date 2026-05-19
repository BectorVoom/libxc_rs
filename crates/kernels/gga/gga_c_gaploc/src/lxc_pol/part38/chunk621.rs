//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 621/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk621<F: Float>(t11218: F, t493: F, t492: F, t169: F, t172: F, t452: F, t10194: F, t10228: F, t10239: F, t10254: F, t10260: F, t10263: F, t105: F, t3532: F, t3537: F, t3542: F, t419: F) -> (F, F, F) {
    let t11219 = t493 * t11218;
    let t11220 = t492 * t11219;
    let t11232 = t11218 * t169 * t172;
    let t11233 = t452 * t11232;
    let t11240 = -F::cast_from(0.28455006635676149599e-1_f64) * t105 * t11220 - F::cast_from(0.85365019907028448797e-1_f64) * t419 * t3537 + F::cast_from(0.56910013271352299198e-1_f64) * t419 * t3542 - F::cast_from(0.47425011059460249332e-2_f64) * t10194 + F::cast_from(0.47425011059460249332e-2_f64) * t10228 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3532 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t11233 - F::cast_from(0.63233348079280332443e-2_f64) * t10239 - F::cast_from(0.47425011059460249332e-2_f64) * t10254 + F::cast_from(0.47425011059460249332e-2_f64) * t10260 + F::cast_from(0.47425011059460249332e-2_f64) * t10263;
    (t11219, t11232, t11240)
}
