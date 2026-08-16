//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1653;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta441(t14365: f64, t25207: f64, t605: f64, t775: f64, t2430: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64, t857: f64, t2656: f64, t7045: f64, t240: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25208, t25211, t25215, t25219, t25222) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1653(t14365, t25207, t605, t775, t2430, t30, t1946, t2684, t7043, t820, t843);
        let (t25223, t25224, t25225, t25227) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1654(t25222, t857, t2656, t7045, t240, t7036);
    (t25208, t25211, t25215, t25219, t25222, t25223, t25224, t25225, t25227)
}
