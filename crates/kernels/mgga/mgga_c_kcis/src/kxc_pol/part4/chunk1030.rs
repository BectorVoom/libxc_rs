//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1030/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1030<F: Float>(t110: F, t1369: F, t1602: F, t1599: F, t4425: F, t4450: F, t25: F, t3977: F, t4434: F, t209: F, t494: F, t617: F, t736: F) -> (F, F, F, F, F) {
    let t12825 = t110 * t1369;
    let t12826 = t12825 * t1602;
    let t12827 = t1599 * t12826;
    let t12829 = t4425 * t4450;
    let t12830 = t1599 * t12829;
    let t12832 = t25 * t3977;
    let t12833 = t12832 * t4434;
    let t12834 = t1599 * t12833;
    let t12838 = t209 * t736 * t494 * t617;
    (t12825, t12827, t12830, t12834, t12838)
}
