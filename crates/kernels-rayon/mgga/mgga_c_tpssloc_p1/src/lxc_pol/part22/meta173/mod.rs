//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1046;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1047;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta173(t25: f64, t1799: f64, t571: f64, t3919: f64, t1408: f64, t3664: f64, t2: f64, t514: f64, t584: f64, t606: f64, t1649: f64, t3672: f64, t517: f64, zeta_threshold: f64, t28: f64, t1081: f64, t157: f64, t182: f64, t172: f64, t1787: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5127, t5131, t5134, t5141, t5142, t5145) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1046(t25, t1799, t571, t3919, t1408, t3664, t2, t514, t584, t606, t1649, t3672, t517, zeta_threshold);
        let t5151 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1047(t28, t1081, t5142, t5145, t584, t157, t5141, zeta_threshold);
        let (t5153, t5154) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1048(t182, t5151, t172, t1787);
    (t5127, t5131, t5134, t5142, t5151, t5153, t5154)
}
