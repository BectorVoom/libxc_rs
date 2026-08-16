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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk880;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk881;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk882;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk883;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk884;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk885;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk886;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta132<F: Float>(t1932: F, t360: F, t3187: F, t3166: F, t383: F, t1003: F, t1058: F, t1061: F, t1063: F, t3076: F, t3180: F, t3186: F, t3189: F, t3193: F, t3197: F, t3200: F, t353: F, t384: F, t1055: F, t1052: F, t1066: F, t3021: F, t3023: F, t3026: F, t3167: F, t3169: F, t3176: F, t388: F, t1068: F, t390: F, t1070: F, t193: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2937: F, t2939: F, t2942: F, t2946: F, t2950: F, t2954: F, t336: F, t25: F, t265: F, t394: F, t2756: F, t1074: F, t2249: F, t2250: F, t396: F, t40: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3201 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk880::<F>(t1932, t360);
        let (t3202, t3204, t3206) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk881::<F>(t3187, t3201, t3166, t383, t1003, t1058, t1061, t1063, t3076, t3180, t3186, t3189, t3193, t3197, t3200, t353, t384);
        let t3207 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk882::<F>(t1055, t3206);
        let (t3209, t3213, t3215) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk883::<F>(t1052, t1066, t3021, t3023, t3026, t3167, t3169, t3176, t3207, t388, t1068, t390);
        let t3216 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk884::<F>(t3215);
        let t3219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk885::<F>(t1070, t193, t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954, t3209, t3213, t3216, t336);
        let (t3220, t3227) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk886::<F>(t25, t265, t394, t2756, t3219, t1074, t2249, t2250, t396, t40, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t3231 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk887::<F>(t2249);
    (t3201, t3202, t3204, t3206, t3207, t3209, t3213, t3215, t3216, t3220, t3227, t3231)
}
