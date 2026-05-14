//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 558/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk558<F: Float>(t2626: F, t2627: F, t1018: F, t446: F, t437: F, t1035: F) -> (F, F, F, F, F) {
    let t2629 = 0.10843581300301739842e-1 * t2626 * t2627;
    let t2630 = t1018 * t446;
    let t2631 = 1.0 / t2630;
    let t2632 = t437 * t2631;
    let t2633 = t1035 * t1035;
    (t2629, t2630, t2631, t2632, t2633)
}
