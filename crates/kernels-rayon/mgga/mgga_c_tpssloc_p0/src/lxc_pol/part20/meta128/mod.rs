//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk837;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk838;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk839;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk840;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk841;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk842;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk843;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta128(t1020: f64, t3103: f64, t1041: f64, t1046: f64, t3039: f64, t3043: f64, t3048: f64, t3054: f64, t3057: f64, t3064: f64, t3070: f64, t3073: f64, t3078: f64, t3084: f64, t3089: f64, t3092: f64, t3094: f64, t3098: f64, t378: f64, t1017: f64, t1030: f64, t1015: f64, t1012: f64, t1009: f64, t990: f64, t1011: f64, t1019: f64, t1004: f64, t1040: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64, t360: f64, t1021: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3104, t3106) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk837(t1020, t3103, t1041, t1046, t3039, t3043, t3048, t3054, t3057, t3064, t3070, t3073, t3078, t3084, t3089, t3092, t3094, t3098, t378);
        let t3108 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk838(t1017, t1030, t1015);
        let t3109 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk839(t1012, t3108);
        let (t3112, t3113, t3114) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk840(t1009, t990, t1011, t1019);
        let t3117 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk841(t1004, t1040);
        let t3120 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk842(t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954);
        let t3121 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk843(t3120, t360);
        let t3123 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk844(t1021, t248, t3121);
    (t3104, t3106, t3108, t3109, t3112, t3113, t3114, t3117, t3120, t3121, t3123)
}
