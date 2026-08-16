//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk432;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk433;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta65(t30: f64, t33: f64, t1340: f64, t762: f64, t531: f64, t566: f64, t513: f64, t605: f64, t516: f64, t1113: f64, zeta_threshold: f64, t212: f64, t555: f64, t225: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t1342, t1343, t1344, t1348, t1353) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk432(t30, t33, t1340, t762, t531, t566, t513, t605, t516, t1113, zeta_threshold);
        let t1357 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk433(t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk434(t225, t561);
    (t1342, t1343, t1344, t1348, t1353, t1357, t1358)
}
