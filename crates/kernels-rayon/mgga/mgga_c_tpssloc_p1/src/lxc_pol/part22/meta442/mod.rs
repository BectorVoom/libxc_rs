//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1788;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1789;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta442(t19676: f64, t19679: f64, t19688: f64, t19699: f64, t225: f64, t1819: f64, t68: f64, t1995: f64, t6330: f64, t1307: f64, t5187: f64, t5279: f64, t1365: f64, t6347: f64, t1347: f64, t19631: f64, t1345: f64, t1348: f64, t1821: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64, t6404: f64, t6408: f64, t6411: f64, t550: f64, t1380: f64, t3792: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19702, t19708, t19715, t19716, t19719) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1788(t19676, t19679, t19688, t19699, t225, t1819, t68, t1995, t6330, t1307, t5187, t5279);
        let (t19725, t19728, t19731) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1789(t1365, t6347, t1307, t1347, t19631, t1345, t1348, t1819, t1821, t19702, t19708, t19716, t19719, t5272, t5278, t5280, t5283, t546, t548, t6404, t6408, t6411);
        let (t19732, t19733, t19735) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1790(t19731, t550, t1380, t3792, t5286);
    (t19702, t19708, t19715, t19716, t19719, t19725, t19728, t19731, t19732, t19733, t19735)
}
