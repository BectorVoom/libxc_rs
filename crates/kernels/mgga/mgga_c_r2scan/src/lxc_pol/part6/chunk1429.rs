//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1429/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1429<F: Float>(t19032: F, t19037: F, t19041: F, t19048: F, t19057: F, t19061: F, t22626: F, t23765: F, t23769: F, t23779: F, t23780: F, t26932: F, t22625: F, t7877: F, t7028: F, t759: F, t761: F) -> (F, F, F) {
    let t26934 = -t19032 + t23765 - t19037 - t23769 + t19041 + t19048 + 0.12154685976e1 * t22626 + 0.285764e-1 * t26932 + t19057 - t19061 + t23779 - t23780;
    let t26938 = t7877 * t22625;
    let t26941 = t759 * t7028 * t761;
    (t26934, t26938, t26941)
}
