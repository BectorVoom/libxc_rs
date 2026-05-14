//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 847/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk847<F: Float>(t1655: F, t38042: F, t38044: F, t38048: F, t38055: F, t38059: F, t38066: F, t38073: F, t38092: F, t38094: F, t38096: F, t38098: F, t38105: F, t39563: F, t7977: F, t8693: F) -> (F,) {
    let t39568 = -0.38514888888888888888e0 * t38092 - 0.25676592592592592592e0 * t38094 + 0.21397160493827160493e0 * t38096 + 0.19257444444444444444e0 * t38098 + 0.86658499999999999998e0 * t38105 + 0.77029777777777777776e0 * t38042 - 0.77029777777777777776e0 * t38044 + 0.11554466666666666666e1 * t38048 - 0.42794320987654320987e0 * t38055 - 0.14443083333333333333e0 * t38059 + 0.19257444444444444444e1 * t38066 - 0.34663399999999999999e1 * t38073 + 0.1056393e1 * t39563 * t1655 - 0.469508e0 * t8693 * t7977;
    (t39568,)
}
