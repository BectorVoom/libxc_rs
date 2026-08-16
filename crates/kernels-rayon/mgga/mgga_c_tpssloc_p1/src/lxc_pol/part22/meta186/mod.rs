//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1106;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta186(t5456: f64, t89: f64, t1458: f64, t1774: f64, t1453: f64, t2331: f64, t1444: f64, t2341: f64, t5396: f64, t95: f64, t1419: f64, tau1: f64, t1449: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5457, t5460, t5464, t5465, t5468, t5469, t5472, t5475) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1106(t5456, t89, t1458, t1774, t1453, t2331, t1444, t2341, t5396, t95, t1419, tau1);
        let (t5480, t5481, t5484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1107(t1449, t2349, t5396);
    (t5457, t5460, t5464, t5465, t5468, t5469, t5472, t5475, t5480, t5481, t5484)
}
