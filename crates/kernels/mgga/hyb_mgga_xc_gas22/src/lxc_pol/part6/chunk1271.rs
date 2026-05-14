//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1271/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1271<F: Float>(t21393: F, t21396: F, t21587: F, t25214: F, t25217: F, t25220: F, t29757: F, t29760: F, t29788: F, t361: F, t21391: F, t950: F, t21390: F, t21425: F, t21427: F, t21430: F, t21433: F) -> (F, F, F, F) {
    let t29792 = 0.621814e-1 * (t21587 - 0.11080740740740740741e0 * t21393 + 0.23744444444444444444e-1 * t21396 - 0.11080740740740740741e0 * t25214 + 0.94977777777777777776e-1 * t25217 - 0.35616666666666666666e-1 * t25220 + 0.23744444444444444444e-1 * t29757 - 0.35616666666666666666e-1 * t29760 + 0.53425e-1 * t29788) * t361;
    let t29818 = t21391 - 56.0 / 27.0 * t21393 + 4.0 / 9.0 * t21396 - 56.0 / 27.0 * t25214 + 16.0 / 9.0 * t25217 - 2.0 / 3.0 * t25220 + 4.0 / 9.0 * t29757 - 2.0 / 3.0 * t29760 + t29788;
    let t29819 = t950 * t29818;
    let t29821 = t21390 - 0.18786444444444444445e1 * t21393 + 0.40256666666666666667e0 * t21396 + t21425 + 0.27595e0 * t21430 - 0.14717333333333333333e1 * t21427 + 0.27595e0 * t21433 - 0.18786444444444444444e1 * t25214 + 0.16102666666666666667e1 * t25217 - 0.60385e0 * t25220 + 0.258925e1 * t29819;
    (t29792, t29818, t29819, t29821)
}
