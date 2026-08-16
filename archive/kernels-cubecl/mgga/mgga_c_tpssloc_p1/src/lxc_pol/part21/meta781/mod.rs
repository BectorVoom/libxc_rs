//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2711;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2712;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2713;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2714;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta781<F: Float>(t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t39360: F, t56140: F, t56141: F, t56147: F, t56149: F, t56150: F, t56151: F, t56152: F, t56159: F, t56160: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t56167: F, t56169: F, t56170: F, t56171: F, t56172: F, t56173: F, t56178: F, t56179: F, t56186: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F, t39496: F, t56202: F, t56203: F, t56207: F, t56208: F, t56219: F, t56279: F, t56298: F, t56299: F, t56351: F, t56362: F, t56363: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t56365: F, t56366: F, t56367: F, t56368: F, t56369: F, t56372: F, t56375: F, t56381: F, t39563: F, t39570: F, t39585: F, t39590: F, t39593: F, t39595: F, t56388: F, t56391: F, t56393: F, t56395: F, t56396: F, t56398: F, t56401: F, t56403: F, t56411: F, t56412: F, t56416: F, t56417: F) -> (F, F, F, F, F) {
        let t57194 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2711::<F>(t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t39360, t56140, t56141, t56147, t56149, t56150, t56151, t56152, t56159, t56160);
        let t57196 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2712::<F>(t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411, t56167, t56169, t56170, t56171, t56172, t56173, t56178, t56179, t56186);
        let t57197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2713::<F>(t39463, t39468, t39472, t39476, t39483, t39490, t39496, t56202, t56203, t56207, t56208, t56219, t56279, t56298, t56299, t56351, t56362, t56363);
        let t57200 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2714::<F>(t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t56365, t56366, t56367, t56368, t56369, t56372, t56375, t56381);
        let t57201 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2715::<F>(t39563, t39570, t39585, t39590, t39593, t39595, t56388, t56391, t56393, t56395, t56396, t56398, t56401, t56403, t56411, t56412, t56416, t56417);
    (t57194, t57196, t57197, t57200, t57201)
}
