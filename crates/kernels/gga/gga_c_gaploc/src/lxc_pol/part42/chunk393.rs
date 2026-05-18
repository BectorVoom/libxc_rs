//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 393/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk393<F: Float>(t3545: F, t492: F, t105: F, t3345: F, t3352: F, t3519: F, t3532: F, t3537: F, t3542: F, t1016: F) -> (F, F, F) {
    let t3546 = t492 * t3545;
    let t3549 = F::new(0.28455006635676149599e-1) * t105 * t3519 + F::new(0.28455006635676149599e-1) * t105 * t3532 + F::new(0.47425011059460249332e-2) * t3345 - F::new(0.85365019907028448797e-1) * t105 * t3537 - F::new(0.47425011059460249332e-2) * t3352 + F::new(0.56910013271352299198e-1) * t105 * t3542 - F::new(0.28455006635676149599e-1) * t105 * t3546;
    let t3553 = t1016 * t1016;
    (t3546, t3549, t3553)
}
