//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1413/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1413<F: Float>(t32422: F, t9851: F, t32401: F, t109539: F, t109541: F, t109817: F, t113769: F, t113772: F, t113779: F, t113792: F, t113796: F, t32395: F, t32498: F, t33802: F, t33808: F, t9519: F, t9544: F, t9855: F) -> (F,) {
    let t115213 = 0.34722222222222222222e-2 * t9851 * t32422;
    let t115215 = 0.34722222222222222222e-2 * t9851 * t32401;
    let t115234 = 0.23148148148148148148e-2 * t109539 + t115213 + t115215 + 0.10416666666666666667e-1 * t33808 * t9544 + 0.52083333333333333333e-2 * t9851 * t32498 + 0.50925925925925925926e-1 * t32395 * t9855 - 0.27777777777777777778e-1 * t33802 * t9544 - 0.27777777777777777778e-1 * t33802 * t9519 + 0.19657407407407407408e-1 * t109817 * t9855 - 0.23214722222222222222e-2 * t113769 - 0.11607361111111111111e-2 * t113772 - 0.15476481481481481481e-2 * t109541 - 0.25794135802469135802e-3 * t113779 - 0.15476481481481481481e-2 * t113792 + 0.23214722222222222222e-2 * t113796;
    (t115234,)
}
