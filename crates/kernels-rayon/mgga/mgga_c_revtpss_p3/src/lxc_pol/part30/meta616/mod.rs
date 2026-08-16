//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2122;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta616(t27123: f64, t7003: f64, t13514: f64, t94: f64, t1937: f64, t27126: f64, t6993: f64, t25178: f64, t7898: f64, t22496: f64, t25082: f64, t32113: f64, t28184: f64, t7235: f64, t2014: f64, t25190: f64, t28176: f64, t1907: f64, t4135: f64, t28196: f64, t28197: f64, t28173: f64, t25188: f64, t7901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98534, t98537, t98539, t98541, t98544) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2122(t27123, t7003, t13514, t94, t1937, t27126, t6993, t25178, t7898, t22496, t25082, t32113);
        let (t98546, t98549, t98553, t98555, t98557) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2123(t28184, t7235, t2014, t25190, t28176, t1907, t4135, t28196, t28197, t28173, t25188, t7901);
    (t98534, t98537, t98539, t98541, t98544, t98546, t98549, t98553, t98555, t98557)
}
