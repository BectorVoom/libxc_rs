//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk141;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk142;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk143;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk144;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk145;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk146;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk147;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta20(t125: f64, t66: f64, t283: f64, t371: f64, t345: f64, t348: f64, t367: f64, t225: f64, t359: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t372 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk141(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk142(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk143(t371, t372, t373);
        let t378 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk144(t345, t348, t367, t375);
        let (t379, t380) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk145(t225, t378, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk146(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk147(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk148(t379, t385);
    (t372, t373, t375, t378, t379, t380, t381, t384, t385, t386)
}
