//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 949/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk949<F: Float>(t11402: F, t1824: F, t22417: F, t1060: F, t16969: F, t11252: F, t11400: F, t1421: F, t16729: F, t16759: F, t16779: F, t16781: F, t16784: F, t22375: F, t22379: F, t22383: F, t22388: F, t22393: F, t22397: F, t22401: F, t22405: F, t22409: F, t22412: F, t22414: F, t22419: F, t5913: F) -> (F, F) {
    let t22423 = t11402 * t22417 * t1824;
    let t22426 = t22417 * t1060;
    let t22427 = t16969 * t22426;
    let t22430 = 0.98556445e-3 * t1421 * t22375 + 0.16426074166666666666e-2 * t1421 * t22379 - 0.10950716111111111111e-2 * t1421 * t22383 - 0.65704296666666666666e-2 * t1421 * t22388 + 0.29201909629629629629e-2 * t1421 * t22393 - 0.43802864444444444444e-2 * t5913 * t22397 + 0.19711289e-2 * t1421 * t22401 - 0.39422578e-2 * t5913 * t22405 - 0.13140859333333333333e-2 * t1421 * t22409 + 0.13140859333333333333e-2 * t22412 - 0.87605728888888888887e-3 * t22414 + t11252 + t16729 + t16759 + t16779 - t16781 - t16784 + 0.98556445e-3 * t11400 * t22419 - 0.19711289e-2 * t11400 * t22423 - 0.39422578e-2 * t11400 * t22427;
    (t22426, t22430)
}
