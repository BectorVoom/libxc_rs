//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1051/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1051<F: Float>(t1993: F, t7528: F, t17016: F, t10527: F, t10532: F, t11209: F, t11211: F, t16702: F, t16705: F, t16714: F, t16719: F, t16983: F, t16986: F, t16989: F, t17002: F, t17018: F, t17020: F, t17022: F, t2030: F, t5348: F, t7645: F) -> (F, F) {
    let t18744 = t7528 * t1993;
    let t18751 = 0.15476481481481481481e-2 * t17016;
    let t18755 = -0.51588271604938271604e-3 * t16702 - 0.23214722222222222222e-2 * t16705 + 0.15476481481481481481e-2 * t10527 - 0.11607361111111111111e-2 * t10532 + 0.38691203703703703703e-3 * t16714 - 0.10446625e-1 * t16719 - 0.17411041666666666666e-2 * t16983 + 0.11607361111111111111e-2 * t16986 - 0.17411041666666666666e-2 * t16989 + 0.77382407407407407406e-3 * t17002 - 0.386e0 * t18744 * t2030 - 0.386e0 * t5348 * t7645 + 0.77382407407407407406e-3 * t11209 - 0.51588271604938271604e-3 * t11211 - t18751 - 0.23214722222222222222e-2 * t17018 + 0.61905925925925925924e-2 * t17020 - 0.41270617283950617282e-2 * t17022;
    (t18744, t18755)
}
