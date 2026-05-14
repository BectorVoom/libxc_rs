//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1415/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1415<F: Float>(t10004: F, t34593: F, t10014: F, t116167: F, t117629: F, t117635: F, t117639: F, t121067: F, t121102: F, t121105: F, t121109: F, t2807: F, t34422: F, t34456: F, t34474: F, t34477: F, t34520: F, t35446: F, t7261: F, t74995: F, t9721: F, t9728: F, t9740: F, t9990: F, t9991: F) -> (F,) {
    let t122456 = t34593 * t10004;
    let t122470 = -t117629 - t117635 - t117639 - 0.10416666666666666667e-1 * t9990 * t34456 * t2807 - 0.51588271604938271603e-3 * t116167 - 0.18571777777777777777e-1 * t121067 - 0.10416666666666666667e-1 * t9740 * t7261 * t34422 * t74995 - 0.10722222222222222222e-1 * t122456 * t9728 + 0.10416666666666666667e-1 * t34474 * t10014 - 0.15476481481481481481e-2 * t121102 + 0.10416666666666666667e-1 * t34477 * t10014 + 0.10416666666666666667e-1 * t9991 * t34520 - 0.10416666666666666667e-1 * t9721 * t35446 - 0.41270617283950617283e-2 * t121105 + 0.77382407407407407407e-3 * t121109;
    (t122470,)
}
