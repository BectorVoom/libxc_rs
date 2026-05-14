//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 490/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk490<F: Float>(t171: F, t1871: F, t584: F, t406: F, t661: F, t1399: F, t1732: F, t1734: F, t1738: F, t1740: F) -> (F, F, F) {
    let t1874 = 0.571528e-1 * t584 * t171 * t1871;
    let t1875 = t406 * t661;
    let t1882 = 0.126595e1 * t1732 - 0.33758666666666666667e1 * t1734 - 0.13651666666666666667e0 * t1738 + 0.27303333333333333333e0 * t1740 + 0.10954222222222222222e0 * t1399;
    (t1874, t1875, t1882)
}
