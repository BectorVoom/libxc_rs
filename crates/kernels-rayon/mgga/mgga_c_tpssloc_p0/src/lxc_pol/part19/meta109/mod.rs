//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk599;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk600;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk601;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk602;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk603;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk604;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta109(t3077: f64, t369: f64, t374: f64, t376: f64, t677: f64, t370: f64, t35: f64, t365: f64, t612: f64, t364: f64, t354: f64, t1032: f64, t1036: f64, t1004: f64, t1031: f64, t1044: f64, t248: f64, t2776: f64, t121: f64, t1023: f64, t1020: f64, t1041: f64, t1046: f64, t3039: f64, t3043: f64, t3048: f64, t3054: f64, t3057: f64, t3064: f64, t3070: f64, t3073: f64, t378: f64, t1017: f64, t1030: f64, t1015: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3078, t3082) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk599(t3077, t369, t374, t376, t677);
        let (t3084, t3087) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk600(t3082, t370, t35, t365, t612);
        let (t3088, t3089, t3092, t3094, t3098) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk601(t3087, t364, t354, t1032, t1036, t1004, t1031, t1044, t248, t2776);
        let t3101 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk602(t121, t376);
        let t3103 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk603(t1023, t248, t3101);
        let t3106 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk604(t1020, t3103, t1041, t1046, t3039, t3043, t3048, t3054, t3057, t3064, t3070, t3073, t3078, t3084, t3089, t3092, t3094, t3098, t378);
        let (t3108, t3109) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk605(t1017, t1030, t1015, t1012);
    (t3078, t3082, t3087, t3088, t3089, t3094, t3098, t3101, t3103, t3106, t3108, t3109)
}
