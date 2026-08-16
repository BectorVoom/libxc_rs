//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta384(t11677: f64, t15027: f64, t3624: f64, t52627: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64, t10477: f64, t1742: f64, t11713: f64, t3503: f64, t1210: f64, t11647: f64, t1731: f64, t11718: f64, t52835: f64, t1744: f64, t11716: f64, t1174: f64, t1725: f64, t2402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52879, t52903, t53079, t53081, t53083) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187(t11677, t15027, t3624, t52627, t1213, t1735, t248, t45017, t10477, t1742, t11713, t3503);
        let (t53087, t53099, t53238, t53274, t53336, t53440) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1188(t11713, t1210, t53081, t11647, t1731, t11718, t52835, t1744, t11716, t1174, t1725, t2402);
    (t52879, t52903, t53079, t53083, t53087, t53099, t53238, t53274, t53336, t53440)
}
