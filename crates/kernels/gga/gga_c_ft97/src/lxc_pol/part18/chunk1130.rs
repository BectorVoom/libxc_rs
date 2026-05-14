//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1130/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1130<F: Float>(t1642: F, t2112: F, t378: F, t9236: F, t23933: F, t376: F, t89: F, t363: F, t590: F, t1369: F, t1370: F, t7943: F, t1882: F, t23906: F, t1374: F, t2999: F) -> (F, F, F, F, F, F, F, F) {
    let t95340 = t1642 * t2112;
    let t95344 = t378 * t9236;
    let t95356 = t89 * t376 * t23933;
    let t95362 = t363 * t590;
    let t95368 = t1369 * t7943 * t1370;
    let t95369 = 14.0 / 27.0 * t95368;
    let t95370 = t1882 * t23906;
    let t95377 = t89 * t2999 * t1374;
    (t95340, t95344, t95356, t95362, t95368, t95369, t95370, t95377)
}
