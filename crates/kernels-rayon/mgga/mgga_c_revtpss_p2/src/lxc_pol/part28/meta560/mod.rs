//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta560(t10073: f64, t25402: f64, t7048: f64, t7056: f64, t233: f64, t41077: f64, t25348: f64, t689: f64, t25411: f64, t1955: f64, t92888: f64, t9646: f64, t1949: f64, t22: f64, t1954: f64, t39643: f64, t25296: f64, t25310: f64, t25313: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93116, t93118, t93123, t93124, t93126, t93134) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2017(t10073, t25402, t7048, t7056, t233, t41077, t25348, t689, t25411, t1955, t92888, t9646);
        let (t93138, t93139, t93142, t93143, t93146) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2018(t1949, t22, t25402, t93134, t1954, t39643, t7056, t25296, t25310, t25313, t686, t72);
    (t93116, t93118, t93123, t93124, t93126, t93138, t93139, t93142, t93143, t93146)
}
