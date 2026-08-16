//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1079;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1080;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta258(t2075: f64, t671: f64, t6548: f64, t6564: f64, t2047: f64, t798: f64, t6579: f64, t6586: f64, t6602: f64, t6617: f64, t6582: f64, t6594: f64, t6607: f64, t6610: f64, t6615: f64, t6622: f64, t218: f64, t2048: f64, t225: f64, t2053: f64, t2718: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7061, t7067, t7069, t7072, t7074, t7076, t7078, t7082, t7084) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1079(t2075, t671, t6548, t6564, t2047, t798, t6579, t6586, t6602, t6617, t6582, t6594, t6607, t6610, t6615, t6622);
        let (t7085, t7087) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1080(t218, t7084, t2048, t225);
        let t7092 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1081(t2053, t2718, t865);
    (t7061, t7067, t7069, t7072, t7074, t7076, t7078, t7082, t7084, t7085, t7087, t7092)
}
