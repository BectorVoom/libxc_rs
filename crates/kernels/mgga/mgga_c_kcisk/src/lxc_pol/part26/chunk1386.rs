//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1386/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1386<F: Float>(t35025: F, t9535: F, t109627: F, t2075: F, t33760: F, t109626: F, t115358: F, t119277: F, t119279: F, t119290: F, t119293: F, t120348: F, t120352: F, t120393: F, t32354: F, t32376: F, t33784: F, t33830: F, t33837: F, t33940: F, t35004: F, t6204: F, t83235: F, t9536: F, t9539: F) -> (F, F) {
    let t120440 = t35025 * t9535;
    let t120448 = t109627 * t2075 * t33760;
    let t120464 = -0.23148148148148148148e-2 * t32354 * t35004 - 0.40208333333333333335e-2 * t115358 * t33837 - 0.17361111111111111111e-2 * t120440 * t9539 - 0.61905925925925925925e-2 * t119277 + 0.15476481481481481481e-2 * t119279 + 0.15476481481481481481e-2 * t119290 - 0.61905925925925925925e-2 * t119293 - 0.34722222222222222222e-2 * t109626 * t120448 - 0.69444444444444444444e-2 * t109626 * t120352 - 0.69444444444444444444e-2 * t109626 * t120348 - 0.34722222222222222223e-2 * t109626 * t120393 - 0.23280625e-2 * t32376 * t33940 * t33784 - 0.10416666666666666667e-1 * t9536 * t6204 * t33830 * t83235;
    (t120448, t120464)
}
