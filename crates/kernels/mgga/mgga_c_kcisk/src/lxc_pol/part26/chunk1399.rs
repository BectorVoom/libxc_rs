//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1399/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1399<F: Float>(t1596: F, t32464: F, t32465: F, t7710: F, t120194: F, t2737: F, t115750: F, t115969: F, t119670: F, t119675: F, t119685: F, t119698: F, t119701: F, t120220: F, t120285: F, t27812: F, t27854: F, t32439: F, t32458: F, t32459: F, t33937: F, t34969: F, t9529: F, t9536: F) -> (F,) {
    let t120794 = t32464 * t32465 * t7710 * t1596;
    let t120812 = t2737 * t120194;
    let t120818 = -0.38801041666666666667e-3 * t33937 * t120220 + 0.67013888888888888888e-3 * t32439 * t120794 + 0.34722222222222222222e-2 * t9536 * t32458 * t32459 * t27854 + 0.69444444444444444444e-2 * t9536 * t115750 * t32459 * t27812 + 0.17361111111111111111e-2 * t9536 * t120794 - 0.30952962962962962963e-2 * t119670 + t115969 - 0.17411041666666666666e-2 * t119675 - 0.23214722222222222222e-2 * t119685 - 0.13888888888888888889e-1 * t9529 * t34969 + 0.17361111111111111111e-2 * t120812 + 0.46429444444444444444e-2 * t119698 - 0.15476481481481481481e-2 * t119701 - 0.60312500000000000001e-2 * t32439 * t120285;
    (t120818,)
}
