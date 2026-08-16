//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta13 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk95;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk96;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk97;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk98;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk99;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk100;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk101;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta13(t154: f64, t205: f64, t131: f64, t206: f64, t119: f64, t209: f64, t191: f64, t218: f64, t144: f64, t186: f64, t189: f64, t202: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t219, t220, t221) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk95(t154, t205, t131, t206, t119, t209);
        let (t222, t225) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk96(t220, t221, t191);
        let t226 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk97(t218, t225);
        let t228 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk98(t144, t186, t189, t225);
        let t229 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk99(t202);
        let t230 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk100(t229, t68);
        let t232 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk101(t228, t230);
        let (t233, t234) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk102(t232);
    (t219, t220, t221, t222, t225, t226, t228, t229, t230, t232, t233, t234)
}
