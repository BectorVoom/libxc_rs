//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2043;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2044;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta468(t16160: f64, t16161: f64, t16163: f64, t16173: f64, t225: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t3734: f64, t1365: f64, t5187: f64, t1307: f64, t3719: f64, t5279: f64, t1347: f64, t16018: f64, t1348: f64, t1819: f64, t1821: f64, t3839: f64, t3844: f64, t3847: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16176, t16186, t16191, t16192, t16195) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2043(t16160, t16161, t16163, t16173, t225, t1345, t68, t1799, t1995, t3734, t1365, t5187);
        let (t16196, t16199, t16202, t16205) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2044(t1307, t16195, t3719, t5279, t1347, t16018, t1345, t1348, t16176, t16186, t16192, t1819, t1821, t3839, t3844, t3847, t5272, t5278, t5280, t5283, t546, t548);
        let t16206 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2045(t16205, t550);
    (t16176, t16186, t16191, t16192, t16196, t16199, t16202, t16205, t16206)
}
