//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk249;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk250;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk251;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk252;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk253;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk254;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta34(t185: f64, t744: f64, t123: f64, t173: f64, t186: f64, t676: f64, t679: f64, t704: f64, t724: f64, t731: f64, t739: f64, t162: f64, t158: f64, t192: f64, t72: f64, t675: f64, t685: f64, t177: f64, t738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t745 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk249(t185);
        let t746 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk250(t744, t745);
        let t749 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk251(t123, t173, t186, t676, t679, t704, t724, t731, t739, t746);
        let t750 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk252(t162, t749);
        let (t751, t755, t757) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk253(t158, t750, t192, t72, t186, t675, t685);
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk254(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk255(t738, t744, t745);
    (t745, t746, t749, t750, t751, t755, t757, t759, t760, t762)
}
