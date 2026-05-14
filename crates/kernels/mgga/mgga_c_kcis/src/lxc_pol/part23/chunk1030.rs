//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1030/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1030<F: Float>(t2484: F, t26550: F, t26527: F, t9042: F, t26553: F, t815: F, t808: F, t9046: F, t2490: F, t62: F, t9047: F, t91794: F, t91796: F, t91799: F, t91801: F, t91804: F, t91806: F, t91809: F, t91811: F, t91814: F) -> (F, F, F, F, F, F) {
    let t91816 = t2484 * t26550;
    let t91818 = t9042 * t26527;
    let t91820 = t815 * t26553;
    let t91822 = t808 * t9046;
    let t91825 = t2490 * t62 * t9047;
    let t91827 = -3.0 / 4.0 * t91794 - 3.0 / 8.0 * t91796 - 3.0 / 4.0 * t91799 + 3.0 / 8.0 * t91801 - 3.0 / 2.0 * t91804 - 3.0 / 8.0 * t91806 + 3.0 / 4.0 * t91809 + 3.0 / 4.0 * t91811 + t91814 / 32.0 + 3.0 / 32.0 * t91816 + 3.0 / 4.0 * t91818 - 15.0 / 8.0 * t91820 - 9.0 / 4.0 * t91822 - 3.0 / 16.0 * t91825;
    (t91816, t91818, t91820, t91822, t91825, t91827)
}
