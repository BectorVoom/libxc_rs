//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 353/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk353<F: Float>(t157: F, t3113: F, t2321: F, t894: F, t882: F, t2334: F, t883: F) -> (F, F, F, F) {
    let t3114 = t157 * t3113;
    let t3122 = t894 * t2321;
    let t3124 = 0.23712505529730124666e-2 * t882 * t3122;
    let t3129 = t883 * t2334;
    (t3114, t3122, t3124, t3129)
}
