//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 781/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk781<F: Float>(t2321: F, t3701: F, t882: F, t11981: F, t874: F) -> (F, F, F) {
    let t13725 = t3701 * t2321;
    let t13726 = t882 * t13725;
    let t13728 = t11981 * t874;
    (t13725, t13726, t13728)
}
