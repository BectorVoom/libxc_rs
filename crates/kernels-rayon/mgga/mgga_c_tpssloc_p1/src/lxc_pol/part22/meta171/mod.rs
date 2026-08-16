//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1037;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta171(t1243: f64, t5000: f64, t1215: f64, t3612: f64, t1755: f64, t1235: f64, t1734: f64, t1246: f64, t491: f64, t5011: f64, t1932: f64, t475: f64, t1751: f64, t493: f64, t5052: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1756: f64, t1758: f64, t3604: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t4964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5064 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1035(t1243, t5000);
        let t5068 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1036(t1215, t3612);
        let (t5069, t5072, t5073, t5075, t5076, t5079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1037(t1755, t5068, t1235, t1734, t1246, t491, t5011, t1215, t1932, t475);
        let (t5080, t5084, t5086, t5088) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1038(t1755, t5079, t1215, t1751, t1246, t493, t5052, t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076);
    (t5064, t5068, t5069, t5072, t5073, t5075, t5076, t5079, t5080, t5084, t5086, t5088)
}
