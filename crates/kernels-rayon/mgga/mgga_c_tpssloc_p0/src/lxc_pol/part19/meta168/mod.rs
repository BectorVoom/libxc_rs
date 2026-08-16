//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk797;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta168(t207: f64, t215: f64, t9569: f64, t2570: f64, t782: f64, t2573: f64, t2690: f64, t59: f64, t154: f64, t2588: f64, t21: f64, t795: f64, t4127: f64, t787: f64, t9526: f64, t9529: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64, t9559: f64, t9561: f64, t9566: f64) -> (f64, f64, f64, f64, f64) {
        let (t9572, t9573, t9574, t9576, t9577, t9579, t9580, t9583) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk797(t207, t215, t9569, t2570, t782, t2573, t2690, t59, t154, t2588, t21, t795);
        let t9584 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk798(t4127, t787, t9526, t9529, t9540, t9542, t9544, t9547, t9552, t9556, t9559, t9561, t9566, t9572, t9574, t9579, t9583);
    (t9573, t9576, t9577, t9580, t9584)
}
