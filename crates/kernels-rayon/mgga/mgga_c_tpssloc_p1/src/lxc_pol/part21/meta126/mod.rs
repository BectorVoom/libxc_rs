//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk849;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk850;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk851;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk852;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta126(t3067: f64, t3068: f64, t1058: f64, t1044: f64, t820: f64, t1023: f64, t884: f64, t225: f64, t3020: f64, t68: f64, t369: f64, t374: f64, t376: f64, t677: f64, t370: f64, t35: f64, t365: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3069 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk849(t3067, t3068);
        let t3070 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk850(t1058, t3069);
        let t3071 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk851(t1044, t820);
        let (t3072, t3073, t3076, t3077, t3078, t3082) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk852(t1023, t884, t3071, t225, t3020, t68, t369, t374, t376, t677);
        let (t3084, t3087) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk853(t3082, t370, t35, t365, t612);
    (t3069, t3070, t3071, t3072, t3073, t3076, t3077, t3078, t3082, t3084, t3087)
}
