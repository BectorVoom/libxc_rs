//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1105;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1106;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta267(t225: f64, t7398: f64, t2061: f64, t213: f64, t2066: f64, t72: f64, t686: f64, t7058: f64, t7064: f64, t886: f64, t7071: f64, t231: f64, t836: f64, t7076: f64, t233: f64, t1957: f64, t1956: f64, t2067: f64, t257: f64, t7067: f64, t7070: f64, t7387: f64, t7390: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7399, t7403) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1104(t225, t7398, t2061, t213);
        let (t7406, t7407) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1105(t2066, t72, t686);
        let (t7409, t7411, t7415, t7419, t7420, t7423, t7424) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1106(t7058, t7407, t7064, t2061, t886, t7071, t231, t836, t7076, t233, t7398, t1957);
        let t7427 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1107(t1956, t2067, t213, t257, t7067, t7070, t7387, t7390, t7399, t7403, t7409, t7411, t7415, t7420, t7424, t887);
    (t7399, t7403, t7406, t7407, t7409, t7411, t7415, t7419, t7420, t7423, t7424, t7427)
}
