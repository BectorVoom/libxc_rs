//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2711;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2712;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2713;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2714;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta781(t39316: f64, t39320: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t39360: f64, t56140: f64, t56141: f64, t56147: f64, t56149: f64, t56150: f64, t56151: f64, t56152: f64, t56159: f64, t56160: f64, t39364: f64, t39373: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t56167: f64, t56169: f64, t56170: f64, t56171: f64, t56172: f64, t56173: f64, t56178: f64, t56179: f64, t56186: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39490: f64, t39496: f64, t56202: f64, t56203: f64, t56207: f64, t56208: f64, t56219: f64, t56279: f64, t56298: f64, t56299: f64, t56351: f64, t56362: f64, t56363: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39518: f64, t39521: f64, t39529: f64, t39539: f64, t39549: f64, t56365: f64, t56366: f64, t56367: f64, t56368: f64, t56369: f64, t56372: f64, t56375: f64, t56381: f64, t39563: f64, t39570: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t56388: f64, t56391: f64, t56393: f64, t56395: f64, t56396: f64, t56398: f64, t56401: f64, t56403: f64, t56411: f64, t56412: f64, t56416: f64, t56417: f64) -> (f64, f64, f64, f64, f64) {
        let t57194 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2711(t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t39360, t56140, t56141, t56147, t56149, t56150, t56151, t56152, t56159, t56160);
        let t57196 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2712(t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411, t56167, t56169, t56170, t56171, t56172, t56173, t56178, t56179, t56186);
        let t57197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2713(t39463, t39468, t39472, t39476, t39483, t39490, t39496, t56202, t56203, t56207, t56208, t56219, t56279, t56298, t56299, t56351, t56362, t56363);
        let t57200 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2714(t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t56365, t56366, t56367, t56368, t56369, t56372, t56375, t56381);
        let t57201 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2715(t39563, t39570, t39585, t39590, t39593, t39595, t56388, t56391, t56393, t56395, t56396, t56398, t56401, t56403, t56411, t56412, t56416, t56417);
    (t57194, t57196, t57197, t57200, t57201)
}
