//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta110 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk606;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk607;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk608;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk609;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk610;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk611;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk612;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta110(t1009: f64, t990: f64, t1011: f64, t1019: f64, t1004: f64, t1040: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64, t360: f64, t1021: f64, t248: f64, t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3112, t3113, t3114) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk606(t1009, t990, t1011, t1019);
        let t3117 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk607(t1004, t1040);
        let t3120 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk608(t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954);
        let t3121 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk609(t3120, t360);
        let t3123 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk610(t1021, t248, t3121);
        let (t3127, t3128) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk611(t1013, t361, t363);
        let (t3129, t3130) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk612(t3037, t3128, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk613(t360);
    (t3112, t3113, t3114, t3117, t3120, t3121, t3123, t3127, t3128, t3129, t3130, t3131)
}
