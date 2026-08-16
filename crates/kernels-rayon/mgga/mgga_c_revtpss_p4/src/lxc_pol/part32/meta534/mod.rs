//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1841;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta534(t1416: f64, t94545: f64, t240: f64, t25981: f64, t25987: f64, t9775: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t25877: f64, t94390: f64, t1399: f64, t2434: f64, t46361: f64, t545: f64, t1032: f64, t9656: f64, t25875: f64, t25894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94546, t94550, t94554, t94564, t94565, t94568) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1841(t1416, t94545, t240, t25981, t25987, t9775, t2453, t4086, t64, t9795, t2018, t40688, t46808);
        let (t94570, t94589, t94633, t94656, t94669, t94674) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1842(t7256, t9784, t25877, t94390, t1399, t2434, t46361, t545, t1032, t9656, t25875, t25894);
    (t94546, t94550, t94554, t94564, t94565, t94568, t94570, t94589, t94633, t94656, t94669, t94674)
}
