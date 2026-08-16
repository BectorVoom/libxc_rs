//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1730;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta485(t1873: f64, t26004: f64, t5690: f64, t7252: f64, t1398: f64, t1903: f64, t543: f64, t1955: f64, t5710: f64, t1513: f64, t25823: f64, t665: f64, t25826: f64, t4287: f64, t6998: f64, t4237: f64, t76: f64, t13269: f64, t38: f64, t1497: f64, t640: f64, t77: f64, t4241: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27955, t27957, t27972, t28008, t28034, t28036) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1730(t1873, t26004, t5690, t7252, t1398, t1903, t543, t1955, t5710, t1513, t25823, t665);
        let (t28037, t28039, t28089, t28093, t28105, t28108) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731(t25826, t28036, t4287, t6998, t4237, t76, t13269, t38, t1497, t640, t77, t4241, t84);
    (t27955, t27957, t27972, t28008, t28034, t28036, t28037, t28039, t28089, t28093, t28105, t28108)
}
