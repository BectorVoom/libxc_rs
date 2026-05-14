//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 303/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk303<F: Float>(t435: F, t10: F, t1023: F, t14: F, t237: F, t799: F, t1024: F, t1026: F) -> (F, F, F, F) {
    let t1028 = f64::sqrt(t435);
    let t1029 = t1028 * t10;
    let t1030 = t1029 * t1023;
    let t1033 = t237 * t14 * t799;
    let t1035 = -0.632975e0 * t1024 - 0.29896666666666666667e0 * t1026 - 0.1023875e0 * t1030 - 0.82156666666666666667e-1 * t1033;
    (t1029, t1030, t1033, t1035)
}
