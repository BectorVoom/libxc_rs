//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 553/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk553<F: Float>(t1636: F, t1785: F, t5015: F, t1849: F, t662: F, t3290: F, t1775: F, t1776: F, t3293: F, t1781: F, t661: F) -> (F, F, F, F, F, F, F) {
    let t5016 = t1636 * t1785;
    let t5017 = t5015 * t5016;
    let t5020 = t662 * t1849;
    let t5021 = t5020 * t3290;
    let t5022 = t1775 * t5021;
    let t5025 = t1776 * t3293;
    let t5026 = t1775 * t5025;
    let t5030 = 1.0 / t1781 / t661;
    (t5016, t5017, t5021, t5022, t5025, t5026, t5030)
}
