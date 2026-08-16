//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk800;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk801;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk802;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk803;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk804;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta141<F: Float>(t1020: F, t3103: F, t1041: F, t1046: F, t3039: F, t3043: F, t3048: F, t3054: F, t3057: F, t3064: F, t3070: F, t3073: F, t3078: F, t3084: F, t3089: F, t3092: F, t3094: F, t3098: F, t378: F, t1017: F, t1030: F, t1015: F, t1012: F, t1009: F, t990: F, t1011: F, t1019: F, t1004: F, t1040: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2937: F, t2939: F, t2942: F, t2946: F, t2950: F, t2954: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3104, t3106) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk800::<F>(t1020, t3103, t1041, t1046, t3039, t3043, t3048, t3054, t3057, t3064, t3070, t3073, t3078, t3084, t3089, t3092, t3094, t3098, t378);
        let (t3107, t3108, t3109) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk801::<F>(t1017, t1030, t1015, t1012);
        let (t3112, t3113, t3114) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk802::<F>(t1009, t990, t1011, t1019);
        let t3117 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk803::<F>(t1004, t1040);
        let t3120 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk804::<F>(t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954);
        let t3121 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk805::<F>(t3120, t360);
    (t3104, t3106, t3107, t3108, t3109, t3112, t3113, t3114, t3117, t3120, t3121)
}
