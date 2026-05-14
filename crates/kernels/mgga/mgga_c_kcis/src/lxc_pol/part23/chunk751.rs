//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 751/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk751<F: Float>(t1559: F, t4355: F, t3938: F, t3947: F, t11407: F, t187: F, t3910: F, t110: F, t1369: F, t1602: F, t1599: F, t4425: F, t4450: F, t25: F, t3977: F, t4434: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12772 = t1559 * t4355;
    let t12780 = t3938 * t3947;
    let t12791 = 0.53272592592592592592e-1 * t11407;
    let t12808 = t187 * t3910;
    let t12825 = t110 * t1369;
    let t12826 = t12825 * t1602;
    let t12827 = t1599 * t12826;
    let t12829 = t4425 * t4450;
    let t12830 = t1599 * t12829;
    let t12832 = t25 * t3977;
    let t12833 = t12832 * t4434;
    (t12772, t12780, t12791, t12808, t12825, t12827, t12830, t12832, t12833)
}
