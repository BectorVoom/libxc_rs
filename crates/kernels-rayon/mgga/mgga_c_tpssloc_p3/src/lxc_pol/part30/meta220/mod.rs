//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1020;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1021;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta220(t2932: f64, t5811: f64, t959: f64, t2980: f64, t5392: f64, t2979: f64, t4514: f64, t4531: f64, t2994: f64, t977: f64, t5398: f64, t978: f64, t3003: f64, t4384: f64, t5718: f64, t5721: f64, t5724: f64, t340: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1020(t2932, t5811, t959, t2980, t5392, t2979, t4514, t4531, t2994, t977, t5398, t978);
        let (t5829, t5836) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1021(t5828, t977, t3003, t4384, t5718, t5721, t5724);
        let (t5837, t5838) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1022(t340, t5836, t343);
    (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828, t5829, t5836, t5837, t5838)
}
