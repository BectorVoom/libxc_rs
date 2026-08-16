//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta457(t378: f64, t53014: f64, t11200: f64, t1678: f64, t11970: f64, t1660: f64, t127: f64, t4823: f64, t11239: f64, t1647: f64, t11245: f64, t11255: f64, t1063: f64, t1592: f64, t247: f64, t42778: f64, t3298: f64, t4746: f64, t4891: f64, t225: f64, t366: f64, t1011: f64, t1655: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53015, t53160, t53326, t53391, t53703, t53704, t53707) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426(t378, t53014, t11200, t1678, t11970, t1660, t127, t4823, t11239, t1647, t11245, t11255);
        let (t53762, t53800, t53877, t53878, t54118) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427(t1063, t1592, t247, t42778, t3298, t4746, t4891, t225, t53014, t366, t1011, t1655, t2438);
    (t53015, t53160, t53326, t53391, t53703, t53704, t53707, t53762, t53800, t53877, t53878, t54118)
}
