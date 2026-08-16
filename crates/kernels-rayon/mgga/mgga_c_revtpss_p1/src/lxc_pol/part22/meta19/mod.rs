//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta19 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk146;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk147;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk148;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk149;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk150;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk151;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk152;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk153;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk154;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk155;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta19(t359: f64, t360: f64, t39: f64, t40: f64, rho0: f64, t351: f64, t335: f64, t72: f64, t245: f64, t125: f64, t66: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t361 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk146(t359, t360);
        let (t362, t365) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk147(t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk148(t361, t365);
        let t367 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk149(t351, t366);
        let t368 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk150(t335);
        let t369 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk151(t368);
        let t370 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk152(t369, t72);
        let t371 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk153(t245, t370);
        let t372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk154(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk155(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk156(t371, t372, t373);
    (t361, t362, t365, t366, t367, t368, t369, t370, t371, t372, t373, t375)
}
