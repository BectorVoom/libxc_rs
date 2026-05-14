//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 315/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk315<F: Float>(t1078: F, t479: F, t1024: F, t1026: F, t1030: F, t1033: F) -> (F, F) {
    let t1079 = t479 * t1078;
    let t1084 = -0.86308333333333333334e0 * t1024 - 0.301925e0 * t1026 - 0.5501625e-1 * t1030 - 0.82785e-1 * t1033;
    (t1079, t1084)
}
