//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2177/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2177<F: Float>(t11496: F, t3448: F, t11502: F, t1184: F, t15418: F, t11571: F, t3447: F, t3469: F, t4899: F, t11570: F, t9288: F, t3450: F, t9258: F) -> (F, F, F, F, F, F, F) {
    let t44517 = t3448 * t11496;
    let t44521 = t3448 * t11502;
    let t44525 = t15418 * t1184;
    let t44527 = t3447 * t44525 * t11571;
    let t44529 = t4899 * t3469;
    let t44536 = t11570 * t9288;
    let t44540 = t3450 * t9258;
    (t44517, t44521, t44525, t44527, t44529, t44536, t44540)
}
