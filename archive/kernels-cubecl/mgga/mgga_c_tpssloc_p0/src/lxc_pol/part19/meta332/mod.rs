//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1192;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta332<F: Float>(t718: F, t9862: F, t157: F, t182: F, t40661: F, t39264: F, t761: F, t2663: F, t9901: F, t2531: F, t9905: F, t39259: F, t2250: F, t2517: F, t707: F, t751: F, t9449: F, t10121: F, t10126: F, t10134: F, t10143: F, t13487: F, t1877: F, t2522: F, t2553: F, t2745: F, t2749: F, t2752: F, t39373: F, t39397: F, t868: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40674, t40677, t40679, t40681, t40683, t40685) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1192::<F>(t718, t9862, t157, t182, t40661, t39264, t761, t2663, t9901, t2531, t9905, t39259);
        let (t40688, t40690, t40705) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193::<F>(t2250, t2517, t707, t751, t9449, t10121, t10126, t10134, t10143, t13487, t1877, t2522, t2553, t2745, t2749, t2752, t39373, t39397, t40674, t40677, t40679, t40681, t40683, t40685, t868);
    (t40674, t40677, t40679, t40681, t40683, t40685, t40688, t40690, t40705)
}
