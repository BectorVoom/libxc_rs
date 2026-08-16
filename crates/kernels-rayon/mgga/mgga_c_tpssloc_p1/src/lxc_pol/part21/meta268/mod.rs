//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1524;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1525;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta268(t1891: f64, t67: f64, t246: f64, t232: f64, t2379: f64, t2628: f64, t835: f64, t812: f64, t2635: f64, t2690: f64, t815: f64, t831: f64, t2617: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9645, t9646) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1524(t1891, t67, t246);
        let (t9647, t9666, t9667, t9668, t9670, t9671) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1525(t232, t2379, t2628, t835, t812, t2635, t2690, t815);
        let (t9672, t9674) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1526(t831, t9671, t2617, t2638);
    (t9645, t9646, t9647, t9666, t9667, t9668, t9670, t9671, t9672, t9674)
}
