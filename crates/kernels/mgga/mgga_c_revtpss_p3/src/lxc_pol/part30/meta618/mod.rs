//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta618<F: Float>(t7234: F, t8995: F, t28199: F, t28021: F, t7235: F, t13648: F, t2014: F, t7312: F, t25861: F, t7732: F, t2322: F, t28056: F, t25194: F, t7898: F, t25851: F, t10416: F, t7735: F, t13435: F, t27137: F, t25856: F, t4248: F, t2034: F, t49564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98590, t98594, t98597, t98599, t98601) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2126::<F>(t7234, t8995, t28199, t28021, t7235, t13648, t2014, t7312, t25861, t7732, t2322, t28056);
        let (t98603, t98605, t98607, t98609, t98611, t98615, t98617) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2127::<F>(t25194, t7898, t25851, t7732, t10416, t7735, t13435, t2322, t27137, t25856, t4248, t2014, t2034, t49564);
    (t98590, t98594, t98597, t98599, t98601, t98603, t98605, t98607, t98609, t98611, t98615, t98617)
}
