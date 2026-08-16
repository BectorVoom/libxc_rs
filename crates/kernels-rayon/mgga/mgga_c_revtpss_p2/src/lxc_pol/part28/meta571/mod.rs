//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta571(t25604: f64, t995: f64, t357: f64, t988: f64, t3046: f64, t7135: f64, t1078: f64, t1982: f64, t3140: f64, t3259: f64, t1032: f64, t7150: f64, t25610: f64, t3093: f64, t4975: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93436, t93437, t93459, t93464, t93484, t93485) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2032(t25604, t995, t357, t988, t3046, t7135, t1078, t1982, t3140, t3259, t1032, t7150);
        let (t93497, t93498) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2033(t25604, t25610, t3093, t4975);
    (t93436, t93437, t93459, t93464, t93484, t93485, t93497, t93498)
}
