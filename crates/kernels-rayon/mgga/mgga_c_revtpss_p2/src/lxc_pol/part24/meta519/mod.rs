//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1543;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1544;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1545;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta519(t24233: f64, t689: f64, t24241: f64, t24249: f64, t24407: f64, t3520: f64, t24294: f64, t698: f64, t24288: f64, t24291: f64, t24274: f64, t24271: f64, t24312: f64, t3390: f64, t24297: f64, t24323: f64, t3435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t81232 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1543(t24233, t689);
        let t81234 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1544(t24241, t689);
        let t81236 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1545(t24249, t689);
        let (t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546(t24407, t3520, t24294, t698, t24288, t24291, t24274, t24271, t24312, t3390, t24297, t24323, t3435);
    (t81232, t81234, t81236, t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650)
}
