//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1295;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta449<F: Float>(t152: F, t185: F, t75836: F, t46125: F, t46130: F, t46132: F, t46134: F, t5398: F, t2658: F, t57897: F, t1484: F, t16606: F, t2522: F, t39249: F, t39256: F, t39309: F, t39312: F, t4314: F, t5527: F, t67239: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t75839, t75840, t75844, t75845, t75846, t75847) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1295::<F>(t152, t185, t75836, t46125, t46130, t46132, t46134, t5398);
        let (t75850, t75851, t75852) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296::<F>(t185, t2658, t75847, t57897, t1484, t16606, t2522, t39249, t39256, t39309, t39312, t4314, t5527, t67239, t75839, t75840, t75844, t75845, t75846);
    (t75839, t75840, t75844, t75845, t75846, t75847, t75850, t75851, t75852)
}
