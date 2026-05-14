//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 202/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk202<F: Float>(t1029: F, t738: F, t1025: F, t270: F, t946: F, t1022: F, t314: F) -> (F, F, F) {
    let t1030 = t738 * t1029;
    let t1033 = 0.76905262301422242837e-2 * t270 * t1025 + 0.64087718584518535698e-3 * t946 - 0.76905262301422242837e-2 * t270 * t1030;
    let t1035 = t314 * t1022;
    (t1030, t1033, t1035)
}
