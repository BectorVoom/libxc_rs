//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1850;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta524(t94564: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t25877: f64, t94390: f64, t1032: f64, t4066: f64, t1955: f64, t1399: f64, t2434: f64, t3924: f64, t676: f64, t46361: f64, t545: f64, t9656: f64, t25875: f64, t25894: f64, t7282: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94565, t94568, t94570, t94589, t94609, t94610) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1850(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t25877, t94390, t1032, t4066, t1955);
        let (t94633, t94639, t94656, t94669, t94674, t94696) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1851(t1399, t2434, t3924, t676, t46361, t545, t1032, t9656, t25875, t25894, t7282, t9646);
    (t94565, t94568, t94570, t94589, t94609, t94610, t94633, t94639, t94656, t94669, t94674, t94696)
}
