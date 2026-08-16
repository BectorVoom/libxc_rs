//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1438/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1438<F: Float>(t122166: F, t6888: F, t6891: F, t115332: F, t7691: F, t6897: F, t8621: F, t90544: F, t22633: F, t22635: F, t31558: F, t97721: F) -> (F, F, F, F) {
    let t122377 = t6888 * t122166 * t6891;
    let t122384 = t6888 * t115332 * t7691;
    let t122390 = t6897 * t90544 * t8621;
    let t122394 = t22633 * t22635 * t31558 * t97721;
    (t122377, t122384, t122390, t122394)
}
