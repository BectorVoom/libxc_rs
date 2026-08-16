//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta701(t15492: f64, t5002: f64, t1174: f64, t18237: f64, t3431: f64, t6187: f64, t698: f64, t1227: f64, t13969: f64, t18341: f64, t18345: f64, t18589: f64, t15743: f64, t5005: f64, t6177: f64, t11692: f64, t11697: f64, t18964: f64, t18583: f64, t3577: f64, t11678: f64, t18367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65998, t66001, t66015, t66024, t66027, t66052) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2286(t15492, t5002, t1174, t18237, t3431, t6187, t698, t1227, t13969, t18341, t18345, t18589);
        let (t66054, t66057, t66073, t66076, t66079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2287(t15743, t5005, t1174, t6177, t698, t11692, t11697, t18964, t18583, t3577, t11678, t18367);
    (t65998, t66001, t66015, t66024, t66027, t66052, t66054, t66057, t66073, t66076, t66079)
}
