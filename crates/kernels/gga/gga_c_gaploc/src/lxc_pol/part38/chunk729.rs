//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 729/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk729<F: Float>(t44376: F, t13296: F, t599: F, t475: F, t3516: F, t874: F) -> (F, F, F, F) {
    let t44377 = 0.47425011059460249332e-2 * t44376;
    let t44381 = t599 * t13296;
    let t44382 = t44381 * t475;
    let t44386 = t3516 * t874;
    (t44377, t44381, t44382, t44386)
}
