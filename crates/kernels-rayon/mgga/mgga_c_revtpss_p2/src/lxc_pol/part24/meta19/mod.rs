//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta19 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk148;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk149;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk150;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk151;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk152;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk153;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk154;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk155;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta19(t335: f64, t72: f64, t245: f64, t125: f64, t66: f64, t283: f64, t345: f64, t348: f64, t367: f64, t225: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t368 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk148(t335);
        let t369 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk149(t368);
        let (t370, t371) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk150(t369, t72, t245);
        let t372 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk151(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk152(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk153(t371, t372, t373);
        let t378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk154(t345, t348, t367, t375);
        let (t379, t380) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk155(t225, t378, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk156(t378, t380);
    (t368, t369, t370, t371, t372, t373, t375, t378, t379, t380, t381)
}
