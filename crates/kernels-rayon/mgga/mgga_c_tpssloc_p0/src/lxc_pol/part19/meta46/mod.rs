//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta46 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk308;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk309;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk310;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk311;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk312;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta46(t893: f64, t914: f64, t880: f64, t886: f64, t307: f64, t302: f64, t906: f64, t897: f64, t902: f64, t910: f64, t310: f64, t324: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t916, t919) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk308(t893, t914, t880, t886);
        let (t922, t923, t924, t931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk309(t307, t302, t880, t906, t886, t897, t902, t910);
        let t932 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk310(t310);
        let t933 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk311(t931, t932);
        let t938 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk312(t880, t886);
        let (t939, t941, t942) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk313(t324, t938, t320);
    (t916, t919, t922, t923, t924, t931, t932, t933, t938, t939, t941, t942)
}
