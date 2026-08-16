//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2198;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta611(t11148: f64, t1227: f64, t248: f64, t45268: f64, t11728: f64, t11729: f64, t3570: f64, t1229: f64, t204: f64, t1090: f64, t3609: f64, t44927: f64, t3623: f64, t11880: f64, t44690: f64, t11913: f64, t11931: f64, t225: f64, t11604: f64, t496: f64, t68: f64, t11601: f64, t11599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45271, t45283, t45293, t45296, t45320) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2198(t11148, t1227, t248, t45268, t11728, t11729, t3570, t1229, t204, t1090, t3609, t44927);
        let (t45323, t45326, t45329, t45345, t45350, t45355, t45375) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2199(t3623, t44927, t11880, t44690, t11913, t11931, t225, t11604, t496, t68, t11601, t11599);
    (t45271, t45283, t45293, t45296, t45320, t45323, t45326, t45329, t45345, t45350, t45355, t45375)
}
