//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1620;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta370(t135: f64, t5889: f64, t973: f64, t5893: f64, t5884: f64, t4593: f64, t4650: f64, t4582: f64, t5398: f64, t607: f64, t4583: f64, t1041: f64, t13948: f64, t13952: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t2960: f64, t3039: f64, t5885: f64, t5890: f64, t5894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17615, t17616, t17620, t17621, t17624, t17625, t17631, t17632, t17635) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1620(t135, t5889, t973, t5893, t5884, t4593, t4650, t4582, t5398, t607);
        let (t17636, t17637, t17640) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1621(t17635, t4583, t4582, t1041, t13948, t13952, t13959, t13963, t13966, t13972, t17616, t17621, t17625, t17632, t2960, t3039, t5885, t5890, t5894);
    (t17615, t17616, t17620, t17621, t17624, t17625, t17631, t17632, t17635, t17636, t17637, t17640)
}
