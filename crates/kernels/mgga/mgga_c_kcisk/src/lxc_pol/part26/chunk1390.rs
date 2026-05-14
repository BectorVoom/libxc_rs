//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1390/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1390<F: Float>(t33794: F, t33883: F, t115676: F, t115679: F, t115684: F, t115693: F, t115695: F, t119385: F, t119388: F, t119399: F, t119402: F, t2740: F, t33808: F, t33854: F, t33864: F, t33960: F, t9850: F, t9851: F, t9869: F) -> (F,) {
    let t120539 = t33794 * t33883;
    let t120553 = -0.23214722222222222222e-2 * t119385 - 0.23214722222222222222e-2 * t119388 + t115676 + t115679 - 0.11574074074074074074e-2 * t120539 + 0.30864197530864197531e-2 * t115684 - 0.15476481481481481481e-2 * t119399 - t115693 - 0.46429444444444444443e-2 * t119402 - 0.10416666666666666667e-1 * t9850 * t33960 * t2740 - t115695 + 0.10416666666666666667e-1 * t33808 * t9869 + 0.10416666666666666667e-1 * t33854 * t9869 + 0.10416666666666666667e-1 * t9851 * t33864;
    (t120553,)
}
