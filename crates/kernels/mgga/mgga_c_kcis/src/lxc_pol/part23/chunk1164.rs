//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1164/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1164<F: Float>(t7968: F, t99059: F, t98794: F, t95157: F, t98767: F, t98781: F, t98784: F, t98787: F, t98790: F, t98797: F, t98800: F, t98804: F, t98863: F, t1615: F, t27614: F, t6176: F, t6183: F) -> (F, F, F) {
    let t99610 = 0.92754700520833333333e-4 * t7968 * t99059;
    let t99615 = 0.10317654320987654321e-2 * t98794;
    let t99619 = 0.17411041666666666666e-2 * t98767 - 0.7722800925925925926e-4 * t95157 - t99610 - 0.11607361111111111111e-2 * t98781 - 0.19345601851851851852e-2 * t98784 + 0.77382407407407407407e-3 * t98787 + 0.12897067901234567901e-2 * t98790 + t99615 - 0.15476481481481481481e-2 * t98797 + 0.77382407407407407406e-3 * t98800 + 0.38691203703703703703e-3 * t98804;
    let t99630 = 0.23214722222222222222e-2 * t98863;
    let t99635 = t6176 * t27614 * t6183 * t1615;
    (t99619, t99630, t99635)
}
