//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 858/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk858<F: Float>(t1299: F, t3483: F, t3739: F, t3779: F, t1412: F, t453: F, t3748: F, t3770: F, t1450: F, t3795: F, t1413: F, t3906: F, t3788: F, t3792: F, t10500: F, t472: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13311 = t3483 * t1299;
    let t13325 = t3739 * t3779;
    let t13327 = t1412 * t1412;
    let t13328 = 1.0 / t13327;
    let t13329 = t453 * t13328;
    let t13330 = t13329 * sigma0;
    let t13344 = t3748 * t3770;
    let t13377 = t3795 * t1450;
    let t13382 = t3906 * t1413;
    let t13383 = t13382 * sigma0;
    let t13387 = t3739 * t3788;
    let t13389 = t3739 * t3792;
    let t13399 = t10500 * t472;
    (t13311, t13325, t13328, t13329, t13330, t13344, t13377, t13382, t13383, t13387, t13389, t13399)
}
