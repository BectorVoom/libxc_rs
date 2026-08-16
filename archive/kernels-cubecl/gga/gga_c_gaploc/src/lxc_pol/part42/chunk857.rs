//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 857/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk857<F: Float>(t15498: F, t15499: F, t44707: F, t590: F, t2679: F, t3626: F, t9800: F, t43446: F, t43454: F, t2639: F, t3614: F, t7284: F, t787: F) -> (F, F, F, F, F) {
    let t45277 = F::cast_from(0.61348681526273199482e1_f64) * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    let t45287 = F::cast_from(0.41708904943825497782e0_f64) * t43446;
    let t45288 = F::cast_from(0.35750489951850426669e0_f64) * t43454;
    let t45298 = F::cast_from(0.25025342966295298669e1_f64) * t787 * t7284 * t3614 * t2639;
    (t45277, t45285, t45287, t45288, t45298)
}
