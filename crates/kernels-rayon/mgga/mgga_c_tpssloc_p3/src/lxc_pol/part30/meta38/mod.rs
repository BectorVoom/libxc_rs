//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta38 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk267;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk268;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk269;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk270;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta38(t730: f64, t731: f64, t177: f64, t172: f64, t688: f64, t690: f64, t694: f64, t699: f64, t180: f64, t118: f64, t168: f64, t181: f64, t677: f64, t680: f64, t705: f64, t725: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t732, t738, t739) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk267(t730, t731, t177);
        let (t740, t745) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk268(t172, t739, t688, t690, t694, t699);
        let t746 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk269(t180);
        let (t747, t750) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk270(t745, t746, t118, t168, t181, t677, t680, t705, t725, t732, t740);
        let t751 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk271(t157, t750);
    (t732, t738, t739, t740, t745, t746, t747, t750, t751)
}
