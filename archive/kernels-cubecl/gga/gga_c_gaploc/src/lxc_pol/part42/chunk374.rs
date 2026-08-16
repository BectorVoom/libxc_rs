//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 374/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk374<F: Float>(t871: F, t931: F, t295: F, t3113: F, t2558: F, t954: F, t943: F, t2571: F, t883: F) -> (F, F, F, F, F) {
    let t3230 = t931 * t871;
    let t3232 = t295 * t3113;
    let t3240 = t954 * t2558;
    let t3242 = F::cast_from(0.64087718584518535698e-3_f64) * t943 * t3240;
    let t3247 = t883 * t2571;
    (t3230, t3232, t3240, t3242, t3247)
}
