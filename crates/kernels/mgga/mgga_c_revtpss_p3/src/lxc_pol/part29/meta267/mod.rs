//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1105;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1106;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta267<F: Float>(t225: F, t7398: F, t2061: F, t213: F, t2066: F, t72: F, t686: F, t7058: F, t7064: F, t886: F, t7071: F, t231: F, t836: F, t7076: F, t233: F, t1957: F, t1956: F, t2067: F, t257: F, t7067: F, t7070: F, t7387: F, t7390: F, t887: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7399, t7403) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1104::<F>(t225, t7398, t2061, t213);
        let (t7406, t7407) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1105::<F>(t2066, t72, t686);
        let (t7409, t7411, t7415, t7419, t7420, t7423, t7424) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1106::<F>(t7058, t7407, t7064, t2061, t886, t7071, t231, t836, t7076, t233, t7398, t1957);
        let t7427 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1107::<F>(t1956, t2067, t213, t257, t7067, t7070, t7387, t7390, t7399, t7403, t7409, t7411, t7415, t7420, t7424, t887);
    (t7399, t7403, t7406, t7407, t7409, t7411, t7415, t7419, t7420, t7423, t7424, t7427)
}
