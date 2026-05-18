//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 180/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk180<F: Float>(t130: F, t139: F, t145: F, t459: F, t464: F, t458: F, t129: F) -> (F, F, F, F, F, F, F) {
    let t860 = t130 * t139;
    let t862 = t860 * t145 * t459;
    let t864 = t464 * t130;
    let t866 = t139 * t145 * t458;
    let t867 = t864 * t866;
    let t869 = F::new(3.0) / F::new(128.0) * t862 - t867 / F::new(128.0);
    let t871 = F::new(1.0) / t129;
    (t860, t862, t864, t866, t867, t869, t871)
}
