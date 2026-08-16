//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2122;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta616<F: Float>(t27123: F, t7003: F, t13514: F, t94: F, t1937: F, t27126: F, t6993: F, t25178: F, t7898: F, t22496: F, t25082: F, t32113: F, t28184: F, t7235: F, t2014: F, t25190: F, t28176: F, t1907: F, t4135: F, t28196: F, t28197: F, t28173: F, t25188: F, t7901: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98534, t98537, t98539, t98541, t98544) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2122::<F>(t27123, t7003, t13514, t94, t1937, t27126, t6993, t25178, t7898, t22496, t25082, t32113);
        let (t98546, t98549, t98553, t98555, t98557) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2123::<F>(t28184, t7235, t2014, t25190, t28176, t1907, t4135, t28196, t28197, t28173, t25188, t7901);
    (t98534, t98537, t98539, t98541, t98544, t98546, t98549, t98553, t98555, t98557)
}
