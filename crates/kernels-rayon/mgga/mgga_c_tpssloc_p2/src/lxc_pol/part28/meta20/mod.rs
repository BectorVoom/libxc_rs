//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk144;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk145;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk146;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk147;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk148;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk149;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta20(t360: f64, sigma0: f64, t34: f64, t35: f64, rho0: f64, t354: f64, t335: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t361, t362, t363) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk144(t360, sigma0);
        let t364 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk145(t362, t363);
        let (t365, t368) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk146(t34, t35, rho0);
        let t369 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk147(t364, t368);
        let (t370, t371) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk148(t354, t369, t335);
        let t372 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk149(t371);
        let t374 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk150(t372, t67, t246);
    (t361, t362, t363, t364, t365, t368, t369, t370, t371, t372, t374)
}
