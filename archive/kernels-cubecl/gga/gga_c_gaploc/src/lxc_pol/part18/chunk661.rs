//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 661/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk661<F: Float>(t4416: F, t823: F, t1422: F, t2031: F, t2021: F, t2200: F, t832: F, t19: F, t2084: F) -> (F, F, F, F) {
    let t5586 = t823 * t4416;
    let t5597 = t2031 * t1422;
    let t5598 = t2021 * t5597;
    let t5629 = t2200 * t832;
    let t5638 = t2084 * t19;
    (t5586, t5598, t5629, t5638)
}
