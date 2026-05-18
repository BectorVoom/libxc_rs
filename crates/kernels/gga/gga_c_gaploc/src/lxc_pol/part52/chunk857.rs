//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 857/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk857<F: Float>(t15498: F, t15499: F, t44707: F, t590: F, t2679: F, t3626: F, t9800: F, t43446: F, t43454: F, t2639: F, t3614: F, t7284: F, t787: F) -> (F, F, F, F, F) {
    let t45277 = F::new(0.61348681526273199482e1) * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    let t45287 = F::new(0.41708904943825497782e0) * t43446;
    let t45288 = F::new(0.35750489951850426669e0) * t43454;
    let t45298 = F::new(0.25025342966295298669e1) * t787 * t7284 * t3614 * t2639;
    (t45277, t45285, t45287, t45288, t45298)
}
