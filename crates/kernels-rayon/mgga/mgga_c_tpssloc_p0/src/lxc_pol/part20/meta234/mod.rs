//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1329;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta234(t2588: f64, t9577: f64, t21: f64, t59: f64, t207: f64, t795: f64, t4127: f64, t787: f64, t9526: f64, t9529: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64, t9559: f64, t9561: f64, t9566: f64, t9572: f64, t9574: f64, t252: f64, t2591: f64, t852: f64, t225: f64, t2711: f64, t2594: f64, t2690: f64, t841: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9579, t9580, t9583, t9584) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1329(t2588, t9577, t21, t59, t207, t795, t4127, t787, t9526, t9529, t9540, t9542, t9544, t9547, t9552, t9556, t9559, t9561, t9566, t9572, t9574);
        let (t9585, t9587, t9590, t9593, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1330(t252, t9584, t2591, t852, t225, t2711, t2594, t2690, t841, t812);
    (t9579, t9580, t9583, t9584, t9585, t9587, t9590, t9593, t9600, t9601)
}
