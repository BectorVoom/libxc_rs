//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk156;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk157;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk158;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk159;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk160;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk161;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta20(t345: f64, t348: f64, t367: f64, t375: f64, t225: f64, t359: f64, t342: f64, t198: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t378 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk156(t345, t348, t367, t375);
        let t379 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk157(t225, t378);
        let t380 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk158(t225, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk159(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk160(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk161(t379, t385);
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk162(t342, t386, t198, t293, t328, t330, t336, t265);
    (t378, t379, t380, t381, t384, t385, t386, t389, t395, t393)
}
