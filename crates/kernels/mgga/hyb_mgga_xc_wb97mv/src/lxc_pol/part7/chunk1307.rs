//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1307/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1307<F: Float>(t31838: F, t31858: F, t31878: F, t31909: F, t938: F, t957: F, t23246: F, t4322: F, t11414: F, t937: F, t958: F, t23180: F, t23183: F, t23254: F, t27021: F, t27024: F, t27027: F, t31779: F, t31782: F, t31810: F, t385: F) -> (F, F, F, F) {
    let t31914 = 1.0 * t938 * (t31838 + t31858 + t31878 + t31909) * t957;
    let t31916 = 0.16081979498692535067e2 * t23246 * t4322;
    let t31917 = t11414 * t937;
    let t31919 = 2.0 * t31917 * t958;
    let t31929 = (t23254 - 0.57685185185185185184e-1 * t23180 + 0.12361111111111111111e-1 * t23183 - 0.57685185185185185187e-1 * t27021 + 0.49444444444444444446e-1 * t27024 - 0.18541666666666666667e-1 * t27027 + 0.12361111111111111111e-1 * t31779 - 0.18541666666666666667e-1 * t31782 + 0.278125e-1 * t31810) * t385;
    (t31914, t31916, t31919, t31929)
}
