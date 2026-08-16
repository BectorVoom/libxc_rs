//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1192;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta332(t718: f64, t9862: f64, t157: f64, t182: f64, t40661: f64, t39264: f64, t761: f64, t2663: f64, t9901: f64, t2531: f64, t9905: f64, t39259: f64, t2250: f64, t2517: f64, t707: f64, t751: f64, t9449: f64, t10121: f64, t10126: f64, t10134: f64, t10143: f64, t13487: f64, t1877: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t2752: f64, t39373: f64, t39397: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40674, t40677, t40679, t40681, t40683, t40685) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1192(t718, t9862, t157, t182, t40661, t39264, t761, t2663, t9901, t2531, t9905, t39259);
        let (t40688, t40690, t40705) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193(t2250, t2517, t707, t751, t9449, t10121, t10126, t10134, t10143, t13487, t1877, t2522, t2553, t2745, t2749, t2752, t39373, t39397, t40674, t40677, t40679, t40681, t40683, t40685, t868);
    (t40674, t40677, t40679, t40681, t40683, t40685, t40688, t40690, t40705)
}
