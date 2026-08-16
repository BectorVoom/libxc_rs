//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta57 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk374;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk375;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk376;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk377;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta57(t555: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t565: f64, t3: f64, t571: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t598: f64, t4: f64, t604: f64, t30: f64, t33: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1433 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk374(t555, t72);
        let (t1436, t1437) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk375(t1432, t1433, t686, t1385, t555);
        let t1450 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk376(t565);
        let (t1458, t1466, t1468) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk377(t3, t571, t578, t582, t586, t590, t594, t598, t4, t604);
        let t1469 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk378(t30, t33, t1468, zeta_threshold);
    (t1433, t1436, t1437, t1450, t1458, t1466, t1468, t1469)
}
