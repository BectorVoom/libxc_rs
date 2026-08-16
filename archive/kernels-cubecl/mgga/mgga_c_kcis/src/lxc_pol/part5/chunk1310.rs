//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1310/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1310<F: Float>(t1430: F, t21106: F, t21110: F, t1437: F, t21073: F, t1330: F, t21078: F, t7164: F, t733: F, t7158: F, t743: F, t21020: F) -> (F, F, F, F, F, F, F) {
    let t21685 = t1430 * t21106;
    let t21688 = t1430 * t21110;
    let t21691 = t1437 * t21073;
    let t21694 = t1330 * t21078;
    let t21704 = t733 * t7164;
    let t21706 = t743 * t7158;
    let t21708 = t1430 * t21020;
    (t21685, t21688, t21691, t21694, t21704, t21706, t21708)
}
