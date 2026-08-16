//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk782;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta170(t207: f64, t215: f64, t9569: f64, t2570: f64, t782: f64, t2690: f64, t59: f64, t154: f64, t2588: f64, t21: f64, t795: f64, t841: f64, t812: f64, t241: f64, t6589: f64, t67: f64, t815: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9572, t9573, t9576, t9577, t9579, t9580, t9583, t9600) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk782(t207, t215, t9569, t2570, t782, t2690, t59, t154, t2588, t21, t795, t841);
        let (t9601, t9607, t9637, t9638) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk783(t812, t9600, t241, t6589, t67, t815, t836);
    (t9572, t9573, t9576, t9577, t9579, t9580, t9583, t9600, t9601, t9607, t9637, t9638)
}
