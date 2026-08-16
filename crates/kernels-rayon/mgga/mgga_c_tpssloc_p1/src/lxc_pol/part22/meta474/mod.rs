//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta474(t20811: f64, t20812: f64, t20821: f64, t20832: f64, t225: f64, t20756: f64, t9946: f64, t4226: f64, t5544: f64, t20800: f64, t824: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t4225: f64, t5601: f64, t5605: f64, t5608: f64, t232: f64, t860: f64, t1509: f64, t5584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20835, t20843, t20846, t20849, t20852) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1869(t20811, t20812, t20821, t20832, t225, t20756, t9946, t4226, t5544, t20800, t824, t1504, t1506, t228, t230, t4225, t5601, t5605, t5608);
        let (t20853, t20854, t20856) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1870(t20852, t232, t860, t1509, t5584);
    (t20835, t20843, t20846, t20849, t20852, t20853, t20854, t20856)
}
