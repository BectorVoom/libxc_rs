//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2206;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta675<F: Float>(t29506: F, t7316: F, t30112: F, t7235: F, t27833: F, t7937: F, t28189: F, t7898: F, t7239: F, t2014: F, t30111: F, t7315: F, t28187: F, t30110: F, t531: F, t7238: F, t28043: F, t7732: F, t28021: F, t28173: F, t1937: F, t75439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109158, t109159, t109162, t109164, t109167, t109169) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2206::<F>(t29506, t7316, t30112, t7235, t27833, t7937, t28189, t7898, t7239, t2014, t30111, t7315);
        let (t109172, t109176, t109178, t109180, t109182, t109194) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2207::<F>(t28187, t7898, t30110, t531, t2014, t7238, t28043, t7732, t28021, t28173, t1937, t75439);
    (t109158, t109159, t109162, t109164, t109167, t109169, t109172, t109176, t109178, t109180, t109182, t109194)
}
