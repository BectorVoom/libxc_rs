//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1397/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1397<F: Float>(t109626: F, t109633: F, t109963: F, t110078: F, t114738: F, t115358: F, t115883: F, t115890: F, t115932: F, t119618: F, t119627: F, t120220: F, t120264: F, t120352: F, t120357: F, t27730: F, t27764: F, t27817: F, t32439: F, t32458: F, t32459: F, t33771: F, t33923: F, t9536: F) -> (F,) {
    let t120760 = -0.34722222222222222222e-2 * t109626 * t120264 - 0.69444444444444444444e-2 * t109626 * t115890 * t27730 - 0.40208333333333333334e-2 * t109633 * t120352 + 0.46296296296296296296e-2 * t109626 * t120357 + 0.46429444444444444443e-2 * t119618 + 0.12897067901234567901e-2 * t119627 - t115883 - 0.41270617283950617283e-2 * t114738 - t115932 + t110078 - 0.20104166666666666667e-2 * t32439 * t120220 + 0.13402777777777777778e-2 * t115358 * t33771 + 0.17361111111111111111e-2 * t9536 * t32458 * t32459 * t27764 + 0.23148148148148148148e-2 * t9536 * t109963 * t33923 * t27817;
    (t120760,)
}
