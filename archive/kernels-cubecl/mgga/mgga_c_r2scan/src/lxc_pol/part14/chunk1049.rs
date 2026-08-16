//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1049/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1049<F: Float>(t37355: F, t597: F, t10673: F, t10682: F, t2279: F, t357: F, t10647: F, t10652: F, t2289: F, t2281: F, t10935: F, t2065: F, t3446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37360 = t597 * t37355;
    let t37362 = t10673 * t10682 * t37360;
    let t37364 = t2279 * t357;
    let t37365 = t37364 * t10647;
    let t37366 = t37365 * t10652;
    let t37368 = t2289 * t357;
    let t37369 = t37368 * t10647;
    let t37370 = t37369 * t10652;
    let t37372 = t2281 * t357;
    let t37373 = t37372 * t10647;
    let t37374 = t37373 * t10652;
    let t37377 = t3446 * t10935 * t2065;
    (t37360, t37362, t37364, t37365, t37366, t37368, t37369, t37370, t37372, t37373, t37374, t37377)
}
