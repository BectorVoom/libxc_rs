//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk328;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk329;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk330;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk331;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk332;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta46(t275: f64, t892: f64, t276: f64, t880: f64, t886: f64, t273: f64, t241: f64, t697: f64, t281: f64, t283: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t893 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk328(t275, t892);
        let t894 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk329(t276);
        let t896 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk330(t880, t886);
        let (t897, t899, t901) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk331(t894, t896, t880, t273);
        let (t902, t904, t906) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk332(t896, t901, t241, t697, t281, t283);
        let (t907, t908) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk333(t906, t241, t340);
    (t893, t894, t896, t897, t899, t901, t902, t904, t906, t907, t908)
}
