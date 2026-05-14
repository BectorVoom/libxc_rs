//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 432/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk432<F: Float>(t1001: F, t1704: F, t286: F, t1700: F, t285: F, t989: F, t991: F, t1009: F, t1022: F, t1662: F, t1021: F, t1020: F, t1646: F, t313: F) -> (F, F, F, F, F, F, F, F) {
    let t1705 = t1001 * t1704;
    let t1706 = t286 * t1705;
    let t1709 = t989 + t991 * t1700 / 288.0 - t285 * t1706 / 96.0;
    let t1710 = t1709 * t1009;
    let t1713 = t1022 * t1662;
    let t1714 = t1021 * t1713;
    let t1715 = t1020 * t1714;
    let t1717 = t313 * t1646;
    (t1705, t1706, t1709, t1710, t1713, t1714, t1715, t1717)
}
