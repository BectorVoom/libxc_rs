//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1764;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1765;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1766;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta484<F: Float>(t25355: F, t789: F, t2471: F, t7018: F, t25331: F, t7058: F, t25309: F, t7063: F, t7060: F, t25296: F, t7064: F, t2435: F, t7015: F, t251: F, t786: F, t1032: F, t2769: F, t233: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25356, t25362, t25364, t25365, t25366, t25368, t25371) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1764::<F>(t25355, t789, t2471, t7018, t25331, t7058, t25309, t7063, t7060, t25296, t7064, t2435, t7015);
        let t25372 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1765::<F>(t251, t786);
        let (t25373, t25374) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1766::<F>(t1032, t2769, t233);
        let t25375 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1767::<F>(t25372, t25374);
    (t25356, t25362, t25364, t25365, t25366, t25368, t25371, t25372, t25373, t25374, t25375)
}
