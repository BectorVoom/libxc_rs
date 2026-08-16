//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1590;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1591;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta445<F: Float>(t18280: F, t3531: F, t6556: F, t6552: F, t3362: F, t5825: F, t606: F, t3417: F, t141: F, t1121: F, t18281: F, t1145: F, t6461: F, t698: F, t6464: F, t6467: F, t6422: F, t689: F, t6426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20256, t20261, t20263, t20266, t20268, t20272, t20273) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1590::<F>(t18280, t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281, t1145);
        let (t20274, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1591::<F>(t141, t20273, t6461, t698, t6464, t6467, t6422, t689);
        let t20285 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1592::<F>(t6426, t689);
    (t20256, t20261, t20263, t20266, t20268, t20272, t20274, t20276, t20278, t20280, t20283, t20285)
}
