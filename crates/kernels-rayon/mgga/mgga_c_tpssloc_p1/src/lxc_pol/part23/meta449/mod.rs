//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1295;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta449(t152: f64, t185: f64, t75836: f64, t46125: f64, t46130: f64, t46132: f64, t46134: f64, t5398: f64, t2658: f64, t57897: f64, t1484: f64, t16606: f64, t2522: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t4314: f64, t5527: f64, t67239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75839, t75840, t75844, t75845, t75846, t75847) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1295(t152, t185, t75836, t46125, t46130, t46132, t46134, t5398);
        let (t75850, t75851, t75852) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296(t185, t2658, t75847, t57897, t1484, t16606, t2522, t39249, t39256, t39309, t39312, t4314, t5527, t67239, t75839, t75840, t75844, t75845, t75846);
    (t75839, t75840, t75844, t75845, t75846, t75847, t75850, t75851, t75852)
}
