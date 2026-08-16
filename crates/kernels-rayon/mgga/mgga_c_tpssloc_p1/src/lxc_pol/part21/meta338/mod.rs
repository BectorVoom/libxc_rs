//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1721;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1722;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1723;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta338(t12645: f64, t12718: f64, t12566: f64, t12568: f64, t12571: f64, t12582: f64, t12585: f64, t12588: f64, t1437: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t3953: f64, t3958: f64, t4021: f64, t605: f64, t645: f64, t86: f64, t9228: f64, t9231: f64, t9239: f64, t5: f64, t112: f64, t111: f64, t4025: f64, t1441: f64, t2319: f64, t649: f64, t671: f64, t2363: f64, t88: f64, t1454: f64, t2281: f64, t4044: f64, t626: f64, t4068: f64, t1453: f64, t2332: f64, t9365: f64, t2331: f64, t4067: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12719, t12722) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1721(t12645, t12718, t12566, t12568, t12571, t12582, t12585, t12588, t1437, t2235, t2240, t2241, t2307, t3953, t3958, t4021, t605, t645, t86, t9228, t9231, t9239);
        let (t12723, t12724, t12725) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1722(t5, t12722, t112, t111, t4025);
        let (t12728, t12734) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1723(t1441, t2319, t649, t671);
        let (t12739, t12747, t12750, t12752, t12754, t12757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1724(t2363, t88, t1454, t2281, t4044, t626, t4068, t1453, t2332, t9365, t2331, t4067);
    (t12719, t12723, t12724, t12725, t12728, t12734, t12739, t12747, t12750, t12752, t12754, t12757)
}
