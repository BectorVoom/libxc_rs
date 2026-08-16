//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta701(t2020: f64, t97804: f64, t15868: f64, t1983: f64, t7753: f64, t22574: f64, t74032: f64, t8643: f64, t28237: f64, t532: f64, t6879: f64, t510: f64, t652: f64, t96729: f64, t1874: f64, t96683: f64, t25992: f64, t7685: f64, t25985: f64, t28821: f64, t7000: f64, t24990: f64, t26167: f64, t7687: f64, t91620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97805, t97808, t97811, t97820, t97829) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197(t2020, t97804, t15868, t1983, t7753, t22574, t74032, t8643, t28237, t532, t6879, t510, t652, t96729);
        let (t97831, t97833, t97835, t97836, t97839, t97842) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198(t1874, t96683, t25992, t7685, t25985, t28821, t7000, t1983, t24990, t26167, t7687, t91620);
    (t97805, t97808, t97811, t97820, t97829, t97831, t97833, t97835, t97836, t97839, t97842)
}
