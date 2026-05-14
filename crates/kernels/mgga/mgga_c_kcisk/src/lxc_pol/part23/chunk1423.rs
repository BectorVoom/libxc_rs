//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1423/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1423<F: Float>(t32353: F, t9850: F, t32342: F, t33941: F, t33766: F, t9523: F, t33822: F, t4419: F, t2737: F, t109738: F, t109760: F, t114162: F, t114172: F, t114188: F, t114215: F, t114223: F, t32339: F, t32359: F, t32380: F, t33827: F, t9519: F, t9539: F, t9851: F, t9869: F) -> (F, F) {
    let t115526 = t9850 * t32353;
    let t115531 = 0.11574074074074074074e-2 * t33941 * t32342;
    let t115535 = t33766 * t9523;
    let t115539 = t4419 * t33822;
    let t115541 = 0.34722222222222222222e-2 * t2737 * t115539;
    let t115548 = 0.18518518518518518518e-1 * t32339 * t33827 - 0.34722222222222222222e-2 * t115526 * t9539 - 0.15476481481481481481e-2 * t114162 - t115531 + 0.69644166666666666664e-2 * t114172 - 0.10416666666666666667e-1 * t9851 * t32380 + 0.40208333333333333334e-2 * t115535 * t9519 + 0.11574074074074074074e-2 * t109738 + t115541 - 0.23214722222222222222e-2 * t114188 - 0.27777777777777777778e-1 * t32359 * t9869 - 0.5787037037037037037e-3 * t109760 + 0.11607361111111111111e-2 * t114215 + 0.46429444444444444444e-2 * t114223;
    (t115539, t115548)
}
