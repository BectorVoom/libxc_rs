//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 460/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk460<F: Float>(t1033: F, t1959: F, t161: F, t8773: F, t1023: F, t1853: F, t1022: F, t2101: F) -> (F, F, F, F) {
    let t8862 = t1033 * t1959;
    let t8878 = t8773 * t161;
    let t8942 = t1023 * t1853;
    let t9014 = t2101 * t1022;
    (t8862, t8878, t8942, t9014)
}
