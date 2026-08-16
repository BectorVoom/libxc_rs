//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk865;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk866;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk867;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk868;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk869;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk870;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk871;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk872;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta133(t1052: f64, t1066: f64, t3021: f64, t3023: f64, t3026: f64, t3167: f64, t3169: f64, t3176: f64, t3207: f64, t388: f64, t1068: f64, t390: f64, t1070: f64, t193: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64, t336: f64, t25: f64, t265: f64, t394: f64, t2756: f64, t1074: f64, t2249: f64, t2250: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1878: f64, t268: f64, t405: f64, t1091: f64, t690: f64, t1229: f64, t154: f64, t636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3209 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk865(t1052, t1066, t3021, t3023, t3026, t3167, t3169, t3176, t3207, t388);
        let (t3213, t3215, t3216) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk866(t1068, t390);
        let t3219 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk867(t1070, t193, t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954, t3209, t3213, t3216, t336);
        let (t3220, t3227) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk868(t25, t265, t394, t2756, t3219, t1074, t2249, t2250, t396, t40, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t3231 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk869(t2249);
        let t3236 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk870(t1878, t268, t405);
        let (t3237, t3238) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk871(t3236, t1091, t690);
        let t3240 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk872(t1229, t154);
        let t3241 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk873(t636);
    (t3209, t3213, t3215, t3216, t3220, t3227, t3231, t3236, t3237, t3238, t3240, t3241)
}
