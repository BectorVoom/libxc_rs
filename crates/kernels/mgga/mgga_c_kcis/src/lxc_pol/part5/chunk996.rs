//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 996/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk996<F: Float>(t12825: F, t1602: F, t1599: F, t25: F, t3977: F, t209: F, t494: F, t617: F, t736: F, t612: F, t110: F, t1611: F) -> (F, F, F, F) {
    let t12826 = t12825 * t1602;
    let t12827 = t1599 * t12826;
    let t12832 = t25 * t3977;
    let t12838 = t209 * t736 * t494 * t617;
    let t12840 = F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t612 * t12838;
    let t12841 = t110 * t1611;
    (t12827, t12832, t12840, t12841)
}
