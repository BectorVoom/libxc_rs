//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2201;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta672(t109077: f64, t2035: f64, t22496: f64, t25082: f64, t33651: f64, t29576: f64, t7235: f64, t2014: f64, t22475: f64, t7312: f64, t2034: f64, t73407: f64, t30122: f64, t32113: f64, t1448: f64, t6781: f64, t28196: f64, t98495: f64, t1353: f64, t28197: f64, t28167: f64, t8717: f64, t25190: f64, t29494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109078, t109081, t109087, t109090, t109092) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2201(t109077, t2035, t22496, t25082, t33651, t29576, t7235, t2014, t22475, t7312, t2034, t73407);
        let (t109095, t109099, t109103, t109107, t109110) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202(t25082, t30122, t32113, t1448, t6781, t28196, t98495, t1353, t28197, t28167, t8717, t2014, t25190, t29494);
    (t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110)
}
