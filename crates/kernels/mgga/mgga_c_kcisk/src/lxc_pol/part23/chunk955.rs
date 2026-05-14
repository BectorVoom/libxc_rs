//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 955/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk955<F: Float>(t2192: F, t3619: F, t5953: F, t3521: F, t5950: F, t5955: F, t459: F, t5684: F, t1175: F, t5926: F, t3587: F, t5927: F, t12847: F, t13179: F, t13183: F, t1421: F, t19327: F, t19331: F, t19334: F, t19339: F, t19343: F, t19348: F, t19354: F, t19359: F, t19364: F, t19368: F, t19372: F, t19376: F, t19380: F, t5913: F) -> (F, F, F, F) {
    let t19381 = t2192 * t3619;
    let t19382 = t5953 * t19381;
    let t19386 = 0.19711289e-2 * t3521 * t5950;
    let t19388 = 0.26281718666666666666e-2 * t3521 * t5955;
    let t19389 = t459 * t5684;
    let t19390 = t19389 * t1175;
    let t19391 = t5926 * t19390;
    let t19394 = t5927 * t3587;
    let t19395 = t5926 * t19394;
    let t19398 = 0.10950716111111111111e-2 * t1421 * t19327 + 0.29201909629629629629e-2 * t1421 * t19331 + 0.43802864444444444444e-2 * t5913 * t19334 + 0.13140859333333333333e-2 * t1421 * t19339 - 0.65704296666666666667e-3 * t1421 * t19343 - 0.10950716111111111111e-2 * t1421 * t19348 + 0.59133867e-2 * t1421 * t19354 + 0.98556445e-3 * t13179 + 0.39422578e-2 * t1421 * t19359 + 0.13140859333333333333e-2 * t13183 - 0.59133867e-2 * t1421 * t19364 - 0.1478346675e-2 * t1421 * t19368 - 0.19711289e-2 * t12847 * t19372 + 0.26281718666666666666e-2 * t12847 * t19376 - t19380 + 0.19711289e-2 * t1421 * t19382 - t19386 + t19388 + 0.1478346675e-2 * t1421 * t19391 + 0.7391733375e-3 * t1421 * t19395;
    (t19381, t19390, t19394, t19398)
}
