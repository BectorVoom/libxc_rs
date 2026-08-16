//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta16 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk122;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk123;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk124;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk125;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk126;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk127;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta16(t121: f64, t154: f64, t40: f64, t268: f64, t119: f64, t133: f64, t134: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t269, t270, t271) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk122(t121, t154, t40);
        let t273 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk123(t268, t269, t271);
        let t275 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk124(t273);
        let t276 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk125(t273);
        let (t279, t281) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk126(t273, t119, t133);
        let (t282, t283) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk127(t134, t241, t271);
        let t285 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk128(t281, t282, t283);
    (t269, t270, t271, t273, t275, t276, t279, t281, t282, t283, t285)
}
