//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 742/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk742<F: Float>(t1424: F, t7: F, t1435: F, t23: F, t34: F, t38: F, t1020: F, t568: F) -> (F, F, F, F, F) {
    let t6658 = t7 * t1424;
    let t6679 = t23 * t1435;
    let t6723 = t34 * t1424;
    let t6738 = t38 * t1435;
    let t6758 = t1020 * t568;
    (t6658, t6679, t6723, t6738, t6758)
}
