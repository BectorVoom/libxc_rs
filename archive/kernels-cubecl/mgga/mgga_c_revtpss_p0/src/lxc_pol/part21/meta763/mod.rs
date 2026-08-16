//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta763 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2710;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta763<F: Float>(t39438: F, t1469: F, t2608: F, t4401: F, t606: F, t10428: F, t4308: F, t14425: F, t705: F, t707: F, t10356: F, t1522: F, t157: F, t30: F, t33: F, t22: F, t39454: F, zeta_threshold: F, t190: F, t706: F, t4398: F, t9387: F, t11061: F, t15071: F, t1583: F, t1940: F, t2411: F, t39442: F, t41154: F, t49872: F, t890: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49873, t49877, t49879, t49882, t49885) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709::<F>(t39438, t1469, t2608, t4401, t606, t10428, t4308, t14425, t705, t707, t10356, t1522, t157);
        let t49889 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2710::<F>(t30, t33, t22, t39454, zeta_threshold);
        let (t49892, t49898, t49903) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711::<F>(t190, t49889, t706, t4398, t9387, t11061, t15071, t1583, t1940, t2411, t39442, t41154, t49872, t49873, t49877, t49879, t49882, t49885, t890);
    (t49873, t49877, t49879, t49882, t49885, t49889, t49892, t49898, t49903)
}
