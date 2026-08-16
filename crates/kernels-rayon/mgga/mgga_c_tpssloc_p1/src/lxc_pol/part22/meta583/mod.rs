//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta583(t374: f64, t485: f64, t486: f64, t9697: f64, t11778: f64, t121: f64, t1229: f64, t204: f64, t1090: f64, t1227: f64, t248: f64, t11880: f64, t44690: f64, t11913: f64, t11604: f64, t496: f64, t68: f64, t107: f64, t9576: f64, t2585: f64, t667: f64, t106: f64, t9364: f64, t35761: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45250, t45268, t45293, t45296, t45326) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093(t374, t485, t486, t9697, t11778, t121, t1229, t204, t1090, t1227, t248, t11880, t44690);
        let (t45329, t45350, t45421, t45422, t45435, t45460) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2094(t11913, t44690, t11604, t496, t68, t107, t9576, t2585, t667, t106, t9364, t35761);
    (t45250, t45268, t45293, t45296, t45326, t45329, t45350, t45421, t45422, t45435, t45460)
}
