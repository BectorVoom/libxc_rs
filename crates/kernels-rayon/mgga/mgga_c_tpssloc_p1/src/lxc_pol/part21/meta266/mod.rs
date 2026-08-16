//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1518;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta266(t2573: f64, t9573: f64, t2690: f64, t59: f64, t154: f64, t2588: f64, t21: f64, t207: f64, t795: f64, t225: f64, t2711: f64, t2594: f64, t841: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9574, t9576, t9577) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1518(t2573, t9573, t2690, t59, t154);
        let (t9579, t9580, t9583, t9590, t9593, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1519(t2588, t9577, t21, t59, t207, t795, t225, t2711, t2594, t2690, t841, t812);
    (t9574, t9576, t9577, t9579, t9580, t9583, t9590, t9593, t9600, t9601)
}
