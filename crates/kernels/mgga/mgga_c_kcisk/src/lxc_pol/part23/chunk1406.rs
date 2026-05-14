//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1406/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1406<F: Float>(t32464: F, t3579: F, t6591: F, t12261: F, t9854: F, t2737: F, t113642: F, t32376: F, t9859: F, t109417: F, t109448: F, t109461: F, t109487: F, t113601: F, t113612: F, t113615: F, t32380: F, t32433: F, t32439: F, t32480: F, t33823: F, t33851: F, t33854: F, t9544: F, t9855: F) -> (F, F, F) {
    let t115017 = t32464 * t6591 * t3579;
    let t115026 = t12261 * t9854;
    let t115027 = t2737 * t115026;
    let t115036 = 0.15476481481481481481e-2 * t113642;
    let t115037 = t32376 * t9859;
    let t115042 = -0.13402777777777777778e-2 * t32439 * t115017 + 0.23214722222222222222e-2 * t113601 + 0.10317654320987654321e-2 * t109417 - 0.11607361111111111111e-2 * t113612 - 0.46429444444444444444e-2 * t113615 - 0.34722222222222222222e-2 * t109448 - 0.34722222222222222222e-2 * t109461 - 0.11574074074074074074e-2 * t115027 - 0.27777777777777777778e-1 * t32480 * t9855 - 0.27777777777777777778e-1 * t33851 * t9544 - 0.10722222222222222222e-1 * t32433 * t33823 - 0.38801041666666666666e-3 * t109487 - t115036 - 0.116403125e-2 * t115037 * t32380 + 0.10416666666666666667e-1 * t33854 * t9544;
    (t115017, t115026, t115042)
}
