//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2027;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta578(t1354: f64, t80991: f64, t1336: f64, t22759: f64, t835: f64, t3795: f64, t22765: f64, t3853: f64, t22704: f64, t22898: f64, t80798: f64, t12248: f64, t6604: f64, t22720: f64, t6883: f64, t22716: f64, t6983: f64, t22742: f64, t6914: f64, t22748: f64, t80727: f64, t22723: f64, t268: f64, t534: f64, t22706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80992, t80998, t81007, t81022, t81027) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2027(t1354, t80991, t1336, t22759, t835, t3795, t22765, t3853, t22704, t22898, t80798, t12248, t6604);
        let (t81037, t81039, t81041, t81043, t81046, t81047) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2028(t22720, t6883, t22716, t6983, t22742, t6914, t22748, t80727, t22723, t268, t534, t22706);
    (t80992, t80998, t81007, t81022, t81027, t81037, t81039, t81041, t81043, t81046, t81047)
}
