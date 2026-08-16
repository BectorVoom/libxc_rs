//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1109;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1110;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta320(t22633: f64, t508: f64, t1501: f64, t5883: f64, t10271: f64, t10273: f64, t10275: f64, t10278: f64, t10280: f64, t10282: f64, t10284: f64, t10287: f64, t10289: f64, t10291: f64, t10295: f64, t1497: f64, t5816: f64, t5872: f64, t1927: f64, t5825: f64, t1486: f64, t5819: f64, t22603: f64, t30: f64, t33: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22634, t22639, t22648) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1109(t22633, t508, t1501, t5883, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289, t10291, t10295);
        let (t22656, t22659, t22662, t22665, t22670) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1110(t1497, t5816, t5872, t1927, t5825, t1486, t5819, t22603);
        let t22671 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1111(t30, t33, t22670, zeta_threshold);
    (t22634, t22639, t22648, t22656, t22659, t22662, t22665, t22670, t22671)
}
