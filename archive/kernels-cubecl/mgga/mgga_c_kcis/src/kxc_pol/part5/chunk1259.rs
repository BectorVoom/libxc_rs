//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1259/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1259<F: Float>(t1396: F, t21020: F, t1395: F, t1394: F, t5477: F, t5632: F, t5780: F, t3738: F, t6913: F, t5628: F, t5748: F, t1464: F) -> (F, F, F, F) {
    let t21021 = t1396 * t21020;
    let t21022 = t1395 * t21021;
    let t21023 = t1394 * t21022;
    let t21025 = t5632 * t5477;
    let t21026 = t1395 * t21025;
    let t21027 = t5780 * t21026;
    let t21029 = t3738 * t6913;
    let t21030 = t1394 * t21029;
    let t21032 = t5748 * t5628;
    let t21033 = t1464 * t21032;
    (t21023, t21027, t21030, t21033)
}
