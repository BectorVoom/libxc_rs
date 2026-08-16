//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta224(t2430: f64, t854: f64, t236: f64, t807: f64, t243: f64, t247: f64, t9949: f64, t237: f64, t9646: f64, t9721: f64, t268: f64, t2479: f64, t2652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10680, t10681, t10682, t10687, t10688, t10689, t10692, t10693) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1014(t2430, t854, t236, t807, t243, t247, t9949, t237, t9646, t9721, t268, t2479, t2652);
    (t10680, t10681, t10682, t10687, t10688, t10689, t10692, t10693)
}
