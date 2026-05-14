//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 367/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk367<F: Float>(t169: F, t3529: F, t172: F, t452: F, t203: F, t3517: F, t492: F, t1339: F, t3516: F) -> (F, F, F, F, F, F) {
    let t3530 = t3529 * t169;
    let t3531 = t3530 * t172;
    let t3532 = t452 * t3531;
    let t3536 = t3517 * t203;
    let t3537 = t492 * t3536;
    let t3541 = t1339 * t3516;
    (t3530, t3531, t3532, t3536, t3537, t3541)
}
