//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2410;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta629(t10108: f64, t257: f64, t68: f64, t2627: f64, t2710: f64, t233: f64, t9970: f64, t2632: f64, t2678: f64, t9975: f64, t2696: f64, t9612: f64, t10021: f64, t812: f64, t841: f64, t849: f64, t23076: f64, t241: f64, t67: f64, t2707: f64, t9601: f64, t2703: f64, t2559: f64, t2570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40890, t40895, t40931, t40933, t40951, t40961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2410(t10108, t257, t68, t2627, t2710, t233, t9970, t2632, t2678, t9975, t2696, t9612);
        let (t40965, t40966, t40971, t40982, t40990, t41008) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2411(t10021, t812, t841, t849, t23076, t241, t67, t2707, t9601, t2703, t2559, t2570);
    (t40890, t40895, t40931, t40933, t40951, t40961, t40965, t40966, t40971, t40982, t40990, t41008)
}
