//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2180;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2181;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta532(t16710: f64, t5057: f64, t689: f64, t12256: f64, t1469: f64, t2251: f64, t12305: f64, t128: f64, t12268: f64, t3360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2180(t16710, t5057, t689);
        let (t16713, t16714, t16715, t16716, t16717) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2181(t16712, t12256, t1469, t2251, t12305, t128);
        let (t16719, t16720, t16721, t16722) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2182(t12268, t1469, t2251, t3360, t128);
    (t16711, t16712, t16713, t16714, t16715, t16716, t16717, t16719, t16720, t16721, t16722)
}
