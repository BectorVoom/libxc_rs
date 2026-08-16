//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk643;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta118(t3270: f64, t3271: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t1100: f64, t407: f64, t1107: f64, t281: f64, t2820: f64, t415: f64, t1114: f64, t699: f64, t1176: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3272, t3279) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk643(t3270, t3271, t3236, t3238, t3245, t3250, t3254);
        let (t3280, t3282, t3287, t3288, t3290, t3293, t3294, t3295, t3297) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk644(t1100, t3279, t3236, t407, t3271, t1107, t281, t2820, t415, t1114, t699, t1176, t241);
    (t3272, t3279, t3280, t3282, t3287, t3288, t3290, t3293, t3294, t3295, t3297)
}
