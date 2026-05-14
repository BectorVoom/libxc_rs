//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 948/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk948<F: Float>(t3521: F, t7005: F, t7009: F, t11252: F, t11255: F, t11257: F, t1421: F, t16725: F, t16729: F, t16732: F, t16736: F, t16741: F, t16745: F, t16749: F, t16752: F, t16755: F, t16759: F, t16760: F, t16764: F, t16767: F, t16771: F, t16775: F, t5913: F) -> (F,) {
    let t16779 = 0.13140859333333333334e-2 * t3521 * t7005;
    let t16781 = 0.8760572888888888889e-3 * t3521 * t7009;
    let t16782 = t11252 + 0.13140859333333333334e-2 * t11255 - 0.8760572888888888889e-3 * t11257 + 0.13140859333333333333e-2 * t1421 * t16725 + t16729 + 0.19711289e-2 * t1421 * t16732 + 0.98556445e-3 * t1421 * t16736 + 0.16426074166666666667e-2 * t1421 * t16741 - 0.65704296666666666667e-3 * t1421 * t16745 + 0.26281718666666666666e-2 * t5913 * t16749 - 0.13140859333333333333e-2 * t1421 * t16752 + 0.52563437333333333332e-2 * t5913 * t16755 + t16759 + 0.10950716111111111111e-2 * t1421 * t16760 + 0.29201909629629629629e-2 * t1421 * t16764 - 0.43802864444444444444e-2 * t5913 * t16767 + 0.98556445e-3 * t1421 * t16771 - 0.39422578e-2 * t5913 * t16775 + t16779 - t16781;
    (t16782,)
}
