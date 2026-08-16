//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2050;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta616(t241: f64, t820: f64, t94491: f64, t5697: f64, t94429: f64, t5701: f64, t27928: f64, t9775: f64, t13775: f64, t25986: f64, t2661: f64, t25978: f64, t5614: f64, t5622: f64, t94443: f64, t13769: f64, t240: f64, t7269: f64, t13760: f64, t25972: f64, t5609: f64, t7028: f64, t9845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98115, t98129, t98131, t98141, t98145, t98146) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2050(t241, t820, t94491, t5697, t94429, t5701, t27928, t9775, t13775, t25986, t2661, t25978, t5614);
        let (t98147, t98148, t98152, t98157, t98161) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2051(t98146, t5622, t94443, t13769, t240, t2661, t7269, t13760, t25972, t5609, t7028, t9845);
    (t98115, t98129, t98131, t98141, t98145, t98147, t98148, t98152, t98157, t98161)
}
