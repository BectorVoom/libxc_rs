//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 186/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk186<F: Float>(t286: F, t708: F, t860: F, t130: F, t713: F, t139: F, t458: F, t295: F, t871: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t924 = t860 * t286 * t708;
    let t926 = t713 * t130;
    let t928 = t139 * t286 * t458;
    let t929 = t926 * t928;
    let t931 = 3.0 / 128.0 * t924 - t929 / 128.0;
    let t933 = t295 * t871;
    let t935 = t931 * t471 + t933 / 2.0;
    (t924, t926, t928, t929, t931, t933, t935)
}
