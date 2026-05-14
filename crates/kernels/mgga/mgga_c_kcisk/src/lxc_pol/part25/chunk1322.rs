//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1322/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1322<F: Float>(t34094: F, t5074: F, t32920: F, t7218: F, t11204: F, t112608: F, t112610: F, t112623: F, t116123: F, t117062: F, t117065: F, t117068: F, t117074: F, t117078: F, t2785: F, t33005: F, t34125: F, t9652: F, t9926: F) -> (F, F) {
    let t117084 = t5074 * t34094;
    let t117086 = t32920 * t7218;
    let t117089 = -0.10416666666666666667e-1 * t11204 * t9926 * t2785 + t117062 + t117065 + 0.33163888888888888888e-2 * t117068 + 0.34722222222222222223e-2 * t112608 - 0.40208333333333333334e-2 * t112610 - 0.17870370370370370371e-2 * t112623 - 0.24872916666666666666e-2 * t117074 - 0.33163888888888888888e-2 * t117078 + 0.55555555555555555558e-1 * t34125 * t33005 - 0.55555555555555555558e-1 * t116123 * t9652 - 0.5895802469135802469e-2 * t117084 - 0.21444444444444444446e-1 * t117086 * t9652;
    (t117084, t117089)
}
