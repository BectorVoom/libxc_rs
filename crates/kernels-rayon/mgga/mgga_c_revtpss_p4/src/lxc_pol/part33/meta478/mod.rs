//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1747;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1748;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1749;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta478(t4003: f64, t6843: f64, t2723: f64, t6016: f64, t197: f64, t531: f64, t2013: f64, t239: f64, t2247: f64, t607: f64, t1927: f64, t644: f64, t7311: f64, t1962: f64, t198: f64, t206: f64, t2411: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23037, t23160, t25081, t25082) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1747(t4003, t6843, t2723, t6016, t197, t531, t2013);
        let (t25137, t25162, t25163, t25190, t25206) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1748(t239, t2247, t607, t1927, t644, t531, t7311, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1749(t2411, t30);
        let (t25220, t25222) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1750(t1946, t2684, t7043, t820, t843);
    (t23037, t23160, t25081, t25082, t25137, t25162, t25163, t25190, t25206, t25207, t25220, t25222)
}
