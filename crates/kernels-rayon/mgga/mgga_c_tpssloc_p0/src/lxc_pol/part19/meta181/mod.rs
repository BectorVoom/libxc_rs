//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta181(t2375: f64, t9879: f64, t2371: f64, t677: f64, t2374: f64, t2535: f64, t2528: f64, t2509: f64, t745: f64, t9843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9881, t9882, t9884, t9885, t9887, t9888, t9890, t9892) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk831(t2375, t9879, t2371, t677, t2374, t2535, t2528, t2509, t745, t9843);
    (t9881, t9882, t9884, t9885, t9887, t9888, t9890, t9892)
}
