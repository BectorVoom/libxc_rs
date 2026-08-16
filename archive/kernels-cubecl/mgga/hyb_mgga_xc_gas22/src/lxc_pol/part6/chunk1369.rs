//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1369/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1369<F: Float>(t21393: F, t21396: F, t21587: F, t25214: F, t25217: F, t25220: F, t29757: F, t29760: F, t29788: F, t361: F, t21391: F, t950: F) -> (F, F, F) {
    let t29792 = F::cast_from(0.621814e-1_f64) * (t21587 - F::cast_from(0.11080740740740740741e0_f64) * t21393 + F::cast_from(0.23744444444444444444e-1_f64) * t21396 - F::cast_from(0.11080740740740740741e0_f64) * t25214 + F::cast_from(0.94977777777777777776e-1_f64) * t25217 - F::cast_from(0.35616666666666666666e-1_f64) * t25220 + F::cast_from(0.23744444444444444444e-1_f64) * t29757 - F::cast_from(0.35616666666666666666e-1_f64) * t29760 + F::cast_from(0.53425e-1_f64) * t29788) * t361;
    let t29818 = t21391 - F::cast_from(56.0_f64) / F::cast_from(27.0_f64) * t21393 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t21396 - F::cast_from(56.0_f64) / F::cast_from(27.0_f64) * t25214 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t25217 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25220 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t29757 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29760 + t29788;
    let t29819 = t950 * t29818;
    (t29792, t29818, t29819)
}
