//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 879/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk879<F: Float>(t1445: F, t1486: F, t1481: F, t3783: F, t13382: F, t492: F, t3507: F, t4229: F, t13328: F, t484: F, t380: F, t470: F, t1413: F, t4295: F, t140: F, t299: F, t4291: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14304 = t1486 * t1445;
    let t14320 = t1481 * t3783;
    let t14340 = t13382 * t492;
    let t14344 = t3507 * t4229;
    let t14364 = t484 * t13328;
    let t14374 = 1.0 / t470 / t380;
    let t14398 = t4295 * t1413;
    let t14399 = t14398 * sigma0;
    let t14405 = t140 * t299 * t4291;
    (t14304, t14320, t14340, t14344, t14364, t14374, t14398, t14399, t14405)
}
