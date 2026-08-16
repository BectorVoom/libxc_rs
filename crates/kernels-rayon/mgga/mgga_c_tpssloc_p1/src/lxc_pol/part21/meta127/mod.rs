//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk854;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk855;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk856;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk857;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta127(t3087: f64, t364: f64, t354: f64, t1032: f64, t1036: f64, t1004: f64, t1031: f64, t1044: f64, t248: f64, t2776: f64, t121: f64, t376: f64, t1023: f64, t1020: f64, t1041: f64, t1046: f64, t3039: f64, t3043: f64, t3048: f64, t3054: f64, t3057: f64, t3064: f64, t3070: f64, t3073: f64, t3078: f64, t3084: f64, t378: f64, t1017: f64, t1030: f64, t1015: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3088, t3089, t3092, t3094, t3098, t3101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk854(t3087, t364, t354, t1032, t1036, t1004, t1031, t1044, t248, t2776, t121, t376);
        let t3103 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk855(t1023, t248, t3101);
        let (t3104, t3106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk856(t1020, t3103, t1041, t1046, t3039, t3043, t3048, t3054, t3057, t3064, t3070, t3073, t3078, t3084, t3089, t3092, t3094, t3098, t378);
        let t3108 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk857(t1017, t1030, t1015);
        let t3109 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk858(t1012, t3108);
    (t3088, t3089, t3092, t3094, t3098, t3101, t3103, t3104, t3106, t3108, t3109)
}
