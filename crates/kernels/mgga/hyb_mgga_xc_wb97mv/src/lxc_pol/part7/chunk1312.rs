//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1312/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1312<F: Float>(t31994: F, t32006: F, t32020: F, t32033: F, t27597: F, t9582: F, t27075: F, t9592: F, t23180: F, t23183: F, t23345: F, t27021: F, t27024: F, t27027: F, t31779: F, t31782: F, t31810: F, t359: F) -> (F, F, F, F) {
    let t32035 = t31994 + t32006 + t32020 + t32033;
    let t32047 = 24.0 * t27597 * t9582;
    let t32049 = 0.38596750796862084161e3 * t27075 * t9592;
    let t32060 = 0.621814e-1 * (t23345 - 0.11080740740740740741e0 * t23180 + 0.23744444444444444444e-1 * t23183 - 0.11080740740740740741e0 * t27021 + 0.94977777777777777776e-1 * t27024 - 0.35616666666666666666e-1 * t27027 + 0.23744444444444444444e-1 * t31779 - 0.35616666666666666666e-1 * t31782 + 0.53425e-1 * t31810) * t359;
    (t32035, t32047, t32049, t32060)
}
