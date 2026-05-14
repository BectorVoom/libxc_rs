//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 462/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk462<F: Float>(t7290: F, t7291: F, t123: F, t2101: F, t161: F, t2610: F, t1959: F, t952: F) -> (F, F, F, F) {
    let t7292 = t7290 * t7291;
    let t7296 = t2101 * t123;
    let t7301 = t161 * t2610;
    let t7324 = t952 * t1959;
    (t7292, t7296, t7301, t7324)
}
