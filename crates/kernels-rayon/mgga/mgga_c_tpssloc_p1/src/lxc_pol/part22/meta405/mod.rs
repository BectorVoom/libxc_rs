//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta405(t11219: f64, t18206: f64, t136: f64, t18211: f64, t3297: f64, t18215: f64, t6014: f64, t699: f64, t1113: f64, t18221: f64, t18225: f64, t6017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18496, t18497, t18499, t18500, t18502, t18503, t18505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704(t11219, t18206, t136, t18211, t3297, t18215, t6014, t699);
        let (t18507, t18508, t18509, t18510, t18512) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1705(t1113, t18221, t136, t18225, t6017, t699);
    (t18496, t18497, t18499, t18500, t18502, t18503, t18505, t18507, t18508, t18509, t18510, t18512)
}
