//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1730;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta385(t16710: f64, t16712: f64, t1130: f64, t5060: f64, t1719: f64, t3432: f64, t12327: f64, t1723: f64, t12331: f64, t3390: f64, t5079: f64, t3407: f64, t5101: f64, t698: f64, t1729: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16821, t16822, t16835, t16840) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1730(t16710, t16712, t1130, t5060, t1719, t3432);
        let (t16851, t16854, t16857, t16862, t16868, t16869, t16873, t16876) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1731(t12327, t1723, t12331, t3390, t5079, t3407, t5101, t698, t16712, t1729, t2439);
    (t16821, t16822, t16835, t16840, t16851, t16854, t16857, t16862, t16868, t16869, t16873, t16876)
}
