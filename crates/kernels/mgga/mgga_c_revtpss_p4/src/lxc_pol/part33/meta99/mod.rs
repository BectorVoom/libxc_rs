//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk629;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk630;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta99<F: Float>(t2435: F, t781: F, t124: F, t68: F, t138: F, t251: F, t785: F, t780: F, t212: F, t860: F, t689: F, t779: F, t887: F, t211: F, t784: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk629::<F>(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk630::<F>(t138, t2438);
        let (t2440, t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk631::<F>(t251, t785, t780, t2439, t212, t860, t689, t779, t887, t211, t784);
    (t2437, t2438, t2439, t2440, t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452)
}
