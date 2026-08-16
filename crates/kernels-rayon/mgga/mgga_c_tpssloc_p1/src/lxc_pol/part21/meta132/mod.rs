//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk880;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk881;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk882;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk883;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk884;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk885;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk886;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta132(t1932: f64, t360: f64, t3187: f64, t3166: f64, t383: f64, t1003: f64, t1058: f64, t1061: f64, t1063: f64, t3076: f64, t3180: f64, t3186: f64, t3189: f64, t3193: f64, t3197: f64, t3200: f64, t353: f64, t384: f64, t1055: f64, t1052: f64, t1066: f64, t3021: f64, t3023: f64, t3026: f64, t3167: f64, t3169: f64, t3176: f64, t388: f64, t1068: f64, t390: f64, t1070: f64, t193: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64, t336: f64, t25: f64, t265: f64, t394: f64, t2756: f64, t1074: f64, t2249: f64, t2250: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3201 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk880(t1932, t360);
        let (t3202, t3204, t3206) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk881(t3187, t3201, t3166, t383, t1003, t1058, t1061, t1063, t3076, t3180, t3186, t3189, t3193, t3197, t3200, t353, t384);
        let t3207 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk882(t1055, t3206);
        let (t3209, t3213, t3215) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk883(t1052, t1066, t3021, t3023, t3026, t3167, t3169, t3176, t3207, t388, t1068, t390);
        let t3216 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk884(t3215);
        let t3219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk885(t1070, t193, t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954, t3209, t3213, t3216, t336);
        let (t3220, t3227) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk886(t25, t265, t394, t2756, t3219, t1074, t2249, t2250, t396, t40, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t3231 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk887(t2249);
    (t3201, t3202, t3204, t3206, t3207, t3209, t3213, t3215, t3216, t3220, t3227, t3231)
}
