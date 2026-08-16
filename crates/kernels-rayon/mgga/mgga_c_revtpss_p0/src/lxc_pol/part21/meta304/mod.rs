//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1559;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta304(t10680: f64, t236: f64, t807: f64, t243: f64, t247: f64, t9949: f64, t237: f64, t9646: f64, t9721: f64, t268: f64, t2479: f64, t2652: f64, t207: f64, t242: f64, t240: f64, t72: f64, t10627: f64, t828: f64, t136: f64, t2476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10681, t10682, t10687, t10688, t10689, t10692, t10693) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1559(t10680, t236, t807, t243, t247, t9949, t237, t9646, t9721, t268, t2479, t2652);
        let (t10696, t10697, t10698, t10700, t10703) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1560(t207, t242, t240, t72, t10627, t828, t136, t2476);
    (t10681, t10682, t10687, t10688, t10689, t10692, t10693, t10696, t10697, t10698, t10700, t10703)
}
