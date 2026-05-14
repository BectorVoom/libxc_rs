//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1330/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1330<F: Float>(t109293: F, t491: F, t6323: F, t14187: F, t32277: F, t6370: F, t13329: F, t21335: F, t19849: F, t4237: F, t1299: F, t2258: F, t9498: F, t1500: F, t394: F, t6309: F) -> (F, F, F, F, F, F, F) {
    let t113359 = t491 * t109293 * t6323;
    let t113362 = t14187 * t32277 * t6370;
    let t113364 = t13329 * t32277;
    let t113365 = t113364 * t21335;
    let t113367 = t19849 * t4237;
    let t113369 = t2258 * t1299;
    let t113370 = t113369 * t9498;
    let t113373 = t1500 * t32277 * t6323;
    let t113375 = t6309 * t394;
    (t113359, t113362, t113365, t113367, t113370, t113373, t113375)
}
