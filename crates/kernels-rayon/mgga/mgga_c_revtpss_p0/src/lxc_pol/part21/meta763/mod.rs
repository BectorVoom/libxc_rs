//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta763 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2710;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta763(t39438: f64, t1469: f64, t2608: f64, t4401: f64, t606: f64, t10428: f64, t4308: f64, t14425: f64, t705: f64, t707: f64, t10356: f64, t1522: f64, t157: f64, t30: f64, t33: f64, t22: f64, t39454: f64, zeta_threshold: f64, t190: f64, t706: f64, t4398: f64, t9387: f64, t11061: f64, t15071: f64, t1583: f64, t1940: f64, t2411: f64, t39442: f64, t41154: f64, t49872: f64, t890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49873, t49877, t49879, t49882, t49885) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709(t39438, t1469, t2608, t4401, t606, t10428, t4308, t14425, t705, t707, t10356, t1522, t157);
        let t49889 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2710(t30, t33, t22, t39454, zeta_threshold);
        let (t49892, t49898, t49903) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711(t190, t49889, t706, t4398, t9387, t11061, t15071, t1583, t1940, t2411, t39442, t41154, t49872, t49873, t49877, t49879, t49882, t49885, t890);
    (t49873, t49877, t49879, t49882, t49885, t49889, t49892, t49898, t49903)
}
