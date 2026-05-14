//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 681/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk681<F: Float>(t3487: F, t5241: F, t16687: F, t19: F, t60: F, t822: F, t16692: F, t201: F, t2536: F, t2925: F, t1022: F, t7275: F, t10938: F, t2021: F, t10007: F, t10627: F) -> (F, F, F, F, F, F, F) {
    let t33308 = t5241 * t3487;
    let t33331 = t822 * t16687 * t19 * t60;
    let t33332 = t201 * t16692;
    let t33348 = t2536 * t2925;
    let t33360 = t7275 * t1022;
    let t33565 = t2021 * t10938;
    let t33601 = t10007 * t10627;
    (t33308, t33331, t33332, t33348, t33360, t33565, t33601)
}
