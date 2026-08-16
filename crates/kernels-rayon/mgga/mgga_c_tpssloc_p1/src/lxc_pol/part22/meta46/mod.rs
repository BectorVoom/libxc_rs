//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk330;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk331;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk332;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk333;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk334;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta46(t290: f64, t912: f64, t893: f64, t880: f64, t886: f64, t307: f64, t302: f64, t906: f64, t897: f64, t902: f64, t910: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t913 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk330(t290);
        let (t914, t916, t917, t919) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk331(t912, t913, t893, t880, t886);
        let (t922, t923) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk332(t307);
        let t924 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk333(t302, t923);
        let (t926, t929, t931) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk334(t880, t906, t886, t897, t902, t910);
        let t932 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk335(t310);
    (t913, t914, t916, t917, t919, t922, t923, t924, t926, t929, t931, t932)
}
