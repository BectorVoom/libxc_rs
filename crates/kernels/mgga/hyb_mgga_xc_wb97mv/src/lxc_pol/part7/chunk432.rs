//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 432/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk432<F: Float>(t1925: F, t1945: F, t1917: F, t1937: F, t622: F, t626: F, t74: F, t79: F, t81: F, t82: F, t1926: F, t1929: F, t615: F, t617: F, t631: F, t72: F, t85: F) -> (F, F, F) {
    let t1946 = t1945 * t1925;
    let t1957 = -2.0 * t1937 * t1925 * t81 + t622 * t1917 * t81 / 2.0 + t1946 * t81 / 4.0 - 4.0 * t1925 * t82 - t79 * t1925 * t81 - 4.0 * t626 * t1917 - t74 * t1917 * t81;
    let t1960 = -t1926 * t81 / 2.0 + 2.0 * t1929 * t1925 - t617 * t1917 + 2.0 * t1917 * t85 + 4.0 * t615 * t631 + 2.0 * t72 * t1957;
    (t1946, t1957, t1960)
}
