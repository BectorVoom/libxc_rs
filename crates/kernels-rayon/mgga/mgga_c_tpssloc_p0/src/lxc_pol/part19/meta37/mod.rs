//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk259;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk260;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk261;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk262;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk263;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk264;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta37(t180: f64, t745: f64, t118: f64, t168: f64, t181: f64, t677: f64, t680: f64, t705: f64, t725: f64, t732: f64, t740: f64, t157: f64, t153: f64, t717: f64, t182: f64, t187: f64, t67: f64, t676: f64, t686: f64, t172: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t746 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk259(t180);
        let t747 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk260(t745, t746);
        let t750 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk261(t118, t168, t181, t677, t680, t705, t725, t732, t740, t747);
        let t751 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk262(t157, t750);
        let (t752, t753, t755, t756, t758) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk263(t153, t751, t157, t717, t182, t187, t67, t181, t676, t686);
        let (t760, t761) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk264(t756, t758, t172, t187);
        let t763 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk265(t739, t745, t746);
    (t746, t747, t750, t751, t752, t753, t755, t756, t758, t760, t761, t763)
}
