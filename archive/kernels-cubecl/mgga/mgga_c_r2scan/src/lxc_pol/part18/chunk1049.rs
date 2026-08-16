//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1049/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1049<F: Float>(t2279: F, t357: F, t10647: F, t10652: F, t2289: F, t2281: F, t1615: F, t2317: F, t269: F, t3438: F, t6855: F, t874: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37364 = t2279 * t357;
    let t37365 = t37364 * t10647;
    let t37366 = t37365 * t10652;
    let t37368 = t2289 * t357;
    let t37369 = t37368 * t10647;
    let t37370 = t37369 * t10652;
    let t37372 = t2281 * t357;
    let t37373 = t37372 * t10647;
    let t37374 = t37373 * t10652;
    let t37386 = t6855 * t1615 * t2317 * t3438 * t269 * t874;
    (t37364, t37365, t37366, t37368, t37369, t37370, t37372, t37373, t37374, t37386)
}
