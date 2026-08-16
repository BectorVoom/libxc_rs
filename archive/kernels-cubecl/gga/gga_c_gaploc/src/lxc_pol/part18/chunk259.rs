//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 259/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk259<F: Float>(t958: F, t959: F, t531: F, t948: F, t808: F, t935: F, t568: F, t325: F, t911: F) -> (F, F, F, F, F) {
    let t960 = t958 * t959;
    let t962 = t531 * t948;
    let t965 = t808 * t935;
    let t966 = t568 * t965;
    let t969 = t911 * t325;
    (t960, t962, t965, t966, t969)
}
