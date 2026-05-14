//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1417/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1417<F: Float>(t2028: F, t7715: F, t112842: F, t33225: F, t34403: F, t7261: F, t9234: F, t112765: F, t117688: F, t117715: F, t117729: F, t117730: F, t117739: F, t117751: F, t117764: F, t117767: F, t117934: F, t22289: F, t22294: F, t33196: F, t34416: F, t34499: F, t34534: F, t34561: F, t35402: F, t9740: F) -> (F, F, F) {
    let t122506 = t7715 * t2028;
    let t122508 = t33225 * t112842 * t122506;
    let t122527 = t7261 * t34403 * t9234 * t2028;
    let t122530 = -0.40208333333333333334e-2 * t112765 * t35402 - 0.34722222222222222222e-2 * t9740 * t122508 - 0.10416666666666666667e-1 * t9740 * t33225 * t34561 * t22289 - 0.13402777777777777778e-2 * t33196 * t122508 - 0.13888888888888888889e-1 * t9740 * t117934 * t34499 * t22294 + 0.34722222222222222223e-2 * t34416 * t34534 - 0.77602083333333333335e-3 * t117688 - t117715 - t117729 - 0.69444444444444444444e-2 * t117730 + t117739 + t117751 + t117764 + t117767 - 0.10416666666666666667e-1 * t9740 * t122527;
    (t122506, t122527, t122530)
}
