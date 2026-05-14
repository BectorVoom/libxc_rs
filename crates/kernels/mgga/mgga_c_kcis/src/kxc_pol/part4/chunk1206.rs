//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1206/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1206<F: Float>(t2080: F, t4332: F, t2084: F, t3919: F, t2072: F, t4330: F, t16144: F, t16048: F, t11409: F, t11411: F, t11413: F, t11415: F, t11455: F, t11457: F, t11460: F, t16050: F, t16062: F, t16088: F) -> (F, F, F, F, F) {
    let t17828 = t2080 * t4332;
    let t17831 = t2084 * t3919;
    let t17834 = t2072 * t4330;
    let t17847 = 0.27785333333333333334e0 * t16144;
    let t17856 = 0.22954444444444444444e0 * t16048;
    let t17861 = -0.45908888888888888888e0 * t11409 + 0.11477222222222222222e0 * t11411 - 0.34431666666666666666e0 * t11413 + 0.17215833333333333333e0 * t11415 + 0.103295e1 * t16088 + 0.20659e1 * t16062 + t17856 - 0.68863333333333333333e0 * t16050 - 0.23154444444444444444e0 * t11455 + 0.69463333333333333333e-1 * t11457 + 0.23154444444444444444e-1 * t11460;
    (t17828, t17831, t17834, t17847, t17861)
}
