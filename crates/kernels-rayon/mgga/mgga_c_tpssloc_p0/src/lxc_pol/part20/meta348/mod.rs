//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1650;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1651;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1652;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta348(t12049: f64, t12095: f64, t12119: f64, t12144: f64, t225: f64, t1995: f64, t68: f64, t1307: f64, t3734: f64, t1365: f64, t3719: f64, t12012: f64, t1347: f64, t1345: f64, t1348: f64, t3839: f64, t3844: f64, t3847: f64, t5278: f64, t546: f64, t548: f64, t550: f64, t1380: f64, t1372: f64, t3787: f64, t3793: f64, t1351: f64, t3791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12147, t12155, t12156) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1650(t12049, t12095, t12119, t12144, t225, t1995, t68, t1307, t3734);
        let (t12157, t12161, t12164, t12167) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1651(t12155, t12156, t1307, t1365, t3719, t12012, t1347, t12147, t1345, t1348, t3839, t3844, t3847, t5278, t546, t548);
        let t12168 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1652(t12167, t550);
        let (t12169, t12172, t12177, t12178) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1653(t12168, t1380, t1372, t3787, t3793, t1351, t3791, t550);
    (t12147, t12156, t12157, t12161, t12164, t12167, t12168, t12169, t12172, t12177, t12178)
}
