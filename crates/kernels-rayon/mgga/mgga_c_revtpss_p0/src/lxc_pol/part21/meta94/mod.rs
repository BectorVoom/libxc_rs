//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk650;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta94(t665: f64, t2339: f64, t613: f64, t99: f64, tau0: f64, t658: f64, t100: f64, t2256: f64, t107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2340, t2341, t2344, t2349) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk650(t665, t2339, t613, t99, tau0);
        let (t2350, t2351, t2354, t2357) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk651(t658, t2349, t100, t2256, t107);
    (t2340, t2341, t2344, t2349, t2350, t2351, t2354, t2357)
}
