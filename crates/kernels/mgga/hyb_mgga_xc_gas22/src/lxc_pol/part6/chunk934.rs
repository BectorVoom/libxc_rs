//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 934/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk934<F: Float>(t6691: F, t8670: F, t8673: F, t8676: F, t8683: F, t8685: F, t8695: F, t8699: F, t8703: F, t8706: F, t8893: F, t8894: F, t8887: F, t829: F, t2229: F, t3316: F) -> (F, F, F) {
    let t8899 = 0.264729375e1 * t8670 - 0.157790625e0 * t8673 + 0.68863333333333333333e0 * t8676 + 0.3529725e1 * t8683 + 0.6311625e0 * t8685 - t6691 - t8893 - t8894 + 0.312585e0 * t8695 + 0.62517e0 * t8699 + 0.312585e0 * t8703 + 0.34731666666666666667e0 * t8706;
    let t8900 = t8887 + t8899;
    let t8901 = t8900 * t829;
    let t8905 = 1.0 * t3316 * t2229;
    (t8900, t8901, t8905)
}
