//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta109 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk740;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk741;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk742;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk743;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk744;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta109(t248: f64, t3051: f64, t884: f64, t1041: f64, t283: f64, t883: f64, t61: f64, t363: f64, t368: f64, t1017: f64, t67: f64, t1058: f64, t1044: f64, t820: f64, t374: f64, t376: f64, t677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3053, t3054, t3061) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk740(t248, t3051, t884, t1041, t283, t883);
        let (t3062, t3067, t3068) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk741(t3061, t61, t363, t368, t1017, t67);
        let t3069 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk742(t3067, t3068);
        let t3070 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk743(t1058, t3069);
        let t3071 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk744(t1044, t820);
        let t3082 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk745(t374, t376, t677);
    (t3053, t3054, t3061, t3062, t3067, t3068, t3069, t3070, t3071, t3082)
}
