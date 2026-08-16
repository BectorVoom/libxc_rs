//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk255;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk256;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk257;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta36(t145: f64, t717: f64, t185: f64, t164: f64, t159: f64, t688: f64, t690: f64, t694: f64, t699: f64, t167: f64, t177: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t718, t719, t723, t724, t725, t730) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk255(t145, t717, t185, t164, t159, t688, t690, t694, t699);
        let t731 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk256(t167);
        let (t732, t738, t739) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk257(t730, t731, t177);
        let (t740, t745) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk258(t172, t739, t688, t690, t694, t699);
    (t718, t719, t723, t724, t725, t730, t731, t732, t738, t739, t740, t745)
}
