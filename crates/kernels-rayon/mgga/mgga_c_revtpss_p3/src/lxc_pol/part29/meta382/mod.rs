//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1374;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1375;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta382(t13610: f64, t13638: f64, t13663: f64, t14308: f64, t1532: f64, t2609: f64, t10437: f64, t2398: f64, t4308: f64, t4305: f64, t262: f64, t4343: f64, t177: f64, t4392: f64, t762: f64, t10605: f64, t162: f64, t4403: f64, t2626: f64, t4398: f64, t10439: f64, t2251: f64, t4402: f64, t2516: f64, t2496: f64, t10443: f64, t10552: f64, t10554: f64, t4541: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14310, t14312, t14313, t14315, t14317, t14318) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1374(t13610, t13638, t13663, t14308, t1532, t2609, t10437, t2398, t4308, t4305, t262, t4343);
        let (t14324, t14327, t14329, t14333, t14334) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1375(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439, t2251, t4402, t2516);
        let (t14335, t14337, t14338) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1376(t14334, t2496, t4398, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14318, t14324, t14327, t14329, t14333, t4541, t775, t9278, t9308, t9316, t9329, t9333);
    (t14310, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t14333, t14335, t14337, t14338)
}
